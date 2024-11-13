/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/swamp-render
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */

use chunk_reader::debug::FileChunkReaderDebug;
use chunk_reader::file::FileChunkReader;
use chunk_reader::ChunkReader;
use chunk_reader::ResourceId;
use std::time::Duration;

#[tokio::test]
async fn load_small_png_debug() {
    let reader = FileChunkReaderDebug::new("assets/".parse().unwrap(), Duration::from_millis(100));

    let found = reader
        .fetch_octets(ResourceId::new("ladder_top.png"))
        .await
        .expect("png file could not be loaded");
    assert_eq!(found.len(), 699);
    assert_eq!(&found[..5], &[0x89, 0x50, 0x4e, 0x47, 0x0d]);
    assert_eq!(&found[696..699], &[0x42, 0x60, 0x82]);
}

#[tokio::test]
async fn load_small_png() {
    let reader = FileChunkReader::new("assets/");

    let found = reader
        .fetch_octets(ResourceId::new("ladder_top.png"))
        .await
        .expect("png file could not be loaded");
    assert_eq!(found.len(), 699);
    assert_eq!(&found[..5], &[0x89, 0x50, 0x4e, 0x47, 0x0d]);
    assert_eq!(&found[696..699], &[0x42, 0x60, 0x82]);
}
