/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/swamp-render
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */

use crate::ResourceId;
use crate::{ChunkReader, ChunkReaderError};
use async_trait::async_trait;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use std::{fs, thread};
use tracing::{debug, warn};

#[derive(Debug)]
pub struct FileChunkReaderDebug {
    prefix: PathBuf,
    /// Duration to sleep before fetching the asset.
    delay: Duration,
}

impl FileChunkReaderDebug {
    /// Creates a new `FileAssetReaderDebug` with the specified delay.
    ///
    /// # Arguments
    ///
    /// * `delay` - The duration to sleep before fetching each asset.
    #[must_use]
    pub fn new(prefix: String, delay: Duration) -> Self {
        Self {
            prefix: prefix.into(),
            delay,
        }
    }
}

#[async_trait(?Send)]
impl ChunkReader for FileChunkReaderDebug {
    async fn fetch_octets(&self, id: ResourceId) -> Result<Vec<u8>, ChunkReaderError> {
        debug!("Starting fetch_octets for path: {}", id.as_str());
        let complete_path = self.prefix.join(id.as_str());
        if !complete_path.exists() {
            warn!("Path does not exist: {:?}", complete_path);
            return Err(ChunkReaderError::ResourceNotFound(id));
        }

        thread::sleep(self.delay);

        let path_arc = Arc::new(complete_path);
        let path_for_task = Arc::clone(&path_arc);

        fs::read(&*path_for_task).map_err(|e| ChunkReaderError::IoError {
            resource: id,
            source: e,
        })
    }
}
