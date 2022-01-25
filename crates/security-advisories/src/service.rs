/*
 * SPDX-License-Identifier: MPL-2.0
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use reqwest::Client;
use std::error::Error;
use std::path::Path;
mod nvd;

pub const CPE_MATCH_FEED: &str = nvd::CPE_MATCH_FEED;
pub const CPE_MATCH_FEED_GZ: &str = nvd::CPE_MATCH_FEED_GZ;

pub async fn fetch_feed_checksum(client: &Client) -> Result<String, Box<dyn Error>> {
    log::info!("fetching CPE match feed checksum ...");
    nvd::fetch_feed_checksum(client).await
}

pub async fn download_cpe_match_feed(
    client: &Client,
    feed_path: &Path,
) -> Result<(), Box<dyn Error>> {
    log::info!("downloading CPE match feed ...");
    nvd::download_cpe_match_feed(client, feed_path).await
}
