/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/swamp-render
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */

use crate::{ChunkReader, ChunkReaderError, ResourceId};
use async_trait::async_trait;
use std::fs;
use std::path::PathBuf;

pub struct FileChunkReader {
    prefix_path: PathBuf,
}

impl FileChunkReader {
    #[must_use]
    pub fn new(prefix: impl Into<PathBuf>) -> Self {
        Self {
            prefix_path: prefix.into(),
        }
    }
}

#[async_trait(?Send)]
impl ChunkReader for FileChunkReader {
    async fn fetch_octets(&self, id: ResourceId) -> Result<Vec<u8>, ChunkReaderError> {
        let full_path = self.prefix_path.join(id.as_str());
        fs::read(&full_path).map_err(|e| ChunkReaderError::IoError {
            resource: id,
            source: e,
        })
    }
}
