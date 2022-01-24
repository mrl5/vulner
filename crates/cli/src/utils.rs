/*
 * SPDX-License-Identifier: MPL-2.0
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use sha2::{Digest, Sha256};
use std::error::Error;
use std::path::Path;
use tokio::fs::File;
use tokio_stream::StreamExt;
use tokio_util::io::ReaderStream;

pub async fn get_file_checksum(path: &Path) -> Result<String, Box<dyn Error>> {
    log::info!("computing checksum of {:?} ...", path.as_os_str());
    let mut hasher = Sha256::new();
    let file = File::open(path).await?;
    let mut stream = ReaderStream::new(file);

    while let Some(chunk) = stream.next().await {
        hasher.update(&chunk?)
    }

    let checksum: String = format!("{:x}", hasher.finalize());
    Ok(checksum)
}
