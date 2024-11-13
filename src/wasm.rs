/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/swamp-render
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */

use crate::{ChunkReader, ChunkReaderError, ResourceId};
use async_trait::async_trait;
use js_sys::{Uint8Array, JSON};
use std::io;
use std::path::PathBuf;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::Response;

/// interface with JavaScript globals
#[wasm_bindgen]
extern "C" {
    type Global;

    #[wasm_bindgen(method, getter, js_name = Window)]
    fn window(this: &Global) -> JsValue;

    #[wasm_bindgen(method, getter, js_name = WorkerGlobalScope)]
    fn worker(this: &Global) -> JsValue;
}

fn js_value_to_err(
    problem: &'static str,
    id: ResourceId,
) -> impl FnOnce(JsValue) -> ChunkReaderError + '_ {
    move |value| {
        let message = JSON::stringify(&value).map_or_else(
            |_| "could not create string from JSON".to_string(),
            |js_str| format!("failed some fetch with {problem}: {js_str}"),
        );

        ChunkReaderError::IoError {
            resource: id,
            source: io::Error::new(io::ErrorKind::Other, message),
        }
    }
}

pub struct HttpWasmChunkReader;

impl HttpWasmChunkReader {
    pub const fn new(_path: &str) -> Self {
        Self
    }
}

#[async_trait(?Send)]
impl ChunkReader for HttpWasmChunkReader {
    async fn fetch_octets(&self, id: ResourceId) -> Result<Vec<u8>, ChunkReaderError> {
        let global: Global = js_sys::global().unchecked_into();
        let promise = if !global.window().is_undefined() {
            let window: web_sys::Window = global.unchecked_into();
            window.fetch_with_str(id.as_str())
        } else if !global.worker().is_undefined() {
            let worker: web_sys::WorkerGlobalScope = global.unchecked_into();
            worker.fetch_with_str(id.as_str())
        } else {
            return Err(ChunkReaderError::IoError {
                resource: id.clone(),
                source: io::Error::new(
                    io::ErrorKind::Other,
                    "could not find a window or a worker from javascript",
                ),
            });
        };

        let resp_value = JsFuture::from(promise)
            .await
            .map_err(js_value_to_err("response future err", id.clone()))?;

        let resp = resp_value
            .dyn_into::<Response>()
            .map_err(js_value_to_err("can not cast to response", id.clone()))?;

        match resp.status() {
            200 => {
                let data = JsFuture::from(resp.array_buffer().unwrap())
                    .await
                    .map_err(js_value_to_err("failed to get array buffer", id))?;
                Ok(Uint8Array::new(&data).to_vec())
            }
            404 => Err(ChunkReaderError::ResourceNotFound(id)),
            status => Err(ChunkReaderError::HttpError(status)),
        }
    }
}
