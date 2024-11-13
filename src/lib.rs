/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/swamp-render
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */

//! A platform-agnostic chunk reader implementation that provides different backends
//! for file system and WebAssembly environments.
//!
//! This crate provides a unified interface for reading chunks of data from either
//! the local filesystem or via HTTP when running in a WebAssembly environment.

pub mod debug;
pub mod file;
#[cfg(target_arch = "wasm32")]
pub mod wasm;

#[cfg(not(target_arch = "wasm32"))]
use crate::file::FileChunkReader;
#[cfg(target_arch = "wasm32")]
use crate::wasm::HttpWasmChunkReader;

use async_trait::async_trait;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResourceId(String);

impl ResourceId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for ResourceId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for ResourceId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

#[derive(Debug)]
pub enum ChunkReaderError {
    ResourceNotFound(ResourceId),
    HttpError(u16),
    IoError {
        resource: ResourceId,
        source: std::io::Error,
    },
}

impl std::error::Error for ChunkReaderError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ChunkReaderError::IoError { source, .. } => Some(source),
            _ => None,
        }
    }
}

impl fmt::Display for ChunkReaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ResourceNotFound(id) => write!(f, "resource not found: {}", id.as_str()),
            Self::HttpError(code) => write!(f, "HTTP error: {}", code),
            Self::IoError { resource, source } => {
                write!(f, "IO error for resource {}: {}", resource.as_str(), source)
            }
        }
    }
}

impl From<std::io::Error> for ChunkReaderError {
    fn from(error: std::io::Error) -> Self {
        Self::IoError {
            resource: ResourceId::new(""),
            source: error,
        }
    }
}

/// Trait for reading chunks of data from a source.
///
/// This trait provides a platform-agnostic way to read data chunks, with different
/// implementations for file system and WebAssembly environments.
#[async_trait(?Send)]
pub trait ChunkReader {
    /// Fetches a chunk of data from the specified resource.
    ///
    /// # Arguments
    /// * `id` - The identifier for the resource to read
    ///
    /// # Returns
    /// * `Ok(Vec<u8>)` - The bytes read from the resource
    /// * `Err(ChunkReaderError)` - If the read operation fails
    async fn fetch_octets(&self, id: ResourceId) -> Result<Vec<u8>, ChunkReaderError>;
}

/// Creates a new platform-specific chunk reader.
///
/// Returns a file-based reader for native platforms and an HTTP-based reader for WebAssembly.
///
/// # Arguments
/// * `path` - The base path for the reader
#[must_use]
pub fn get_platform_reader(path: &str) -> Box<dyn ChunkReader> {
    #[cfg(not(target_arch = "wasm32"))]
    return Box::new(FileChunkReader::new(path));
    #[cfg(target_arch = "wasm32")]
    return Box::new(HttpWasmChunkReader::new(path));
}
