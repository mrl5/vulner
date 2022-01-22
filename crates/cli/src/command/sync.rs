/*
 * SPDX-License-Identifier: MPL-2.0
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use security_advisories::http::get_client;
use security_advisories::service::{download_cpe_match_feed, fetch_feed_checksum, CPE_MATCH_FEED};
use std::error::Error;
use std::io;
use std::path::{Path, PathBuf};
use tokio::fs::create_dir_all;
use tokio::join;
use tokio::process::Command;

pub async fn execute(feed_path: PathBuf) -> Result<(), Box<dyn Error>> {
    let client = get_client()?;
    create_dir_all(&feed_path).await?;

    let file_path = feed_path.join(CPE_MATCH_FEED.split(".gz").next().unwrap());
    if !file_path.exists() {
        download_cpe_match_feed(&client, &feed_path).await?;
        gunzip(&feed_path.join(CPE_MATCH_FEED)).await?;
    } else {
        let remote_checksum = fetch_feed_checksum(&client);
        let fs_checksum = get_cpe_feed_checksum(&file_path);

        let (remote_checksum, fs_checksum) = join!(remote_checksum, fs_checksum);
        if remote_checksum? != fs_checksum? {
            download_cpe_match_feed(&client, &feed_path).await?;
            gunzip(&feed_path.join(CPE_MATCH_FEED)).await?;
        }
    }

    println!(
        "CPE match feed is up to date, available in {:?}",
        file_path.as_os_str()
    );
    Ok(())
}

async fn get_cpe_feed_checksum(path: &Path) -> Result<String, Box<dyn Error>> {
    log::info!("computing checksum of {:?} ...", path.as_os_str());
    let cmd = "/usr/bin/sha256sum";
    let output = Command::new(cmd).arg(path).output().await?;

    if !output.status.success() {
        handle_process_err(output.status.code(), cmd)?;
    }

    let checksum = String::from_utf8(output.stdout)?;
    Ok(checksum.split(' ').next().unwrap().to_owned())
}

async fn gunzip(target: &Path) -> Result<(), Box<dyn Error>> {
    log::info!("uncompressing {:?} ...", target.as_os_str());
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

    println!("Uncompressed {}", CPE_MATCH_FEED);
    Ok(())
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
