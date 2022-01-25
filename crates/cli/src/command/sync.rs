/*
 * SPDX-License-Identifier: MPL-2.0
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::utils::{get_file_checksum, gunzip};
use security_advisories::http::get_client;
use security_advisories::service::{
    download_cpe_match_feed, fetch_feed_checksum, CPE_MATCH_FEED, CPE_MATCH_FEED_GZ,
};
use std::error::Error;
use std::path::PathBuf;
use tokio::fs::create_dir_all;
use tokio::join;

pub async fn execute(feed_path: PathBuf) -> Result<(), Box<dyn Error>> {
    let client = get_client()?;
    create_dir_all(&feed_path).await?;

    let file_path = feed_path.join(CPE_MATCH_FEED);
    if !file_path.exists() {
        log::info!(
            "CPE feed not present in {:?}, downloading ...",
            file_path.as_os_str()
        );
        download_cpe_match_feed(&client, &feed_path).await?;
        gunzip(&feed_path.join(CPE_MATCH_FEED_GZ)).await?;
    } else {
        let remote_checksum = fetch_feed_checksum(&client);
        let fs_checksum = get_file_checksum(&file_path);

        let (remote_checksum, fs_checksum) = join!(remote_checksum, fs_checksum);
        if remote_checksum? != fs_checksum? {
            println!(
                "New version of {} is available, downloading ...",
                CPE_MATCH_FEED
            );
            download_cpe_match_feed(&client, &feed_path).await?;
            gunzip(&feed_path.join(CPE_MATCH_FEED_GZ)).await?;
        }
    }

    println!(
        "CPE match feed is up to date, available in {:?}",
        file_path.as_os_str()
    );
    Ok(())
}
