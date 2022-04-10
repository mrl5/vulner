/*
 * SPDX-License-Identifier: MPL-2.0
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use security_advisories::service::CPE_MATCH_FEED;
use sha2::{Digest, Sha256};
use std::error::Error;
use std::io;
use std::path::{Path, PathBuf};
use tokio::fs::File;
use tokio::process::Command;
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

pub async fn gunzip(target: &Path) -> Result<(), Box<dyn Error>> {
    // todo: rust native
    println!("Uncompressing {:?} ...", target.as_os_str());
    let cmd = "/bin/gunzip";
    let status = Command::new(cmd)
        .arg("-f")
        .arg(target)
        .spawn()?
        .wait()
        .await?;

    if !status.success() {
        handle_process_err(status.code(), cmd)?
    }

    log::debug!("uncompressed {:?}", target.as_os_str());
    Ok(())
}

pub fn get_feed_path(feed_dir: &Path) -> PathBuf {
    let feed = feed_dir.join(CPE_MATCH_FEED);
    if !feed.exists() {
        log::error!(
            "{:?} doesn't exist. Did you forget to run `vulner sync`?",
            feed.as_os_str()
        );
    }
    feed
}

fn handle_process_err(code: Option<i32>, process_name: &str) -> Result<(), Box<dyn Error>> {
    match code {
        Some(c) => {
            let err_kind = io::ErrorKind::Other;
            let err_msg = format!("{} process exited with code {}", process_name, c);
            Err(Box::new(io::Error::new(err_kind, err_msg)))
        }
        None => {
            let err_kind = io::ErrorKind::Interrupted;
            let err_msg = format!("{} process terminated by signal", process_name);
            Err(Box::new(io::Error::new(err_kind, err_msg)))
        }
    }
}
