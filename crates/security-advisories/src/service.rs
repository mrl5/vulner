/*
 * SPDX-License-Identifier: MPL-2.0
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::cve_summary::CveSummary;
use reqwest::Client;
use serde_json::Value;
use std::error::Error;
use std::path::Path;
mod cisa;
mod nvd;

pub const CPE_MATCH_FEED: &str = nvd::CPE_MATCH_FEED;
pub const CPE_MATCH_FEED_GZ: &str = nvd::CPE_MATCH_FEED_GZ;

pub trait ApiKeys {
    fn get_nvd_api_key(&self) -> Option<String>;
}

pub async fn fetch_cves_by_cpe(
    client: &Client,
    cpe: &str,
    api_keys: &'_ dyn ApiKeys,
) -> Result<Value, Box<dyn Error>> {
    log::debug!("fetching CVEs by CPE ...");
    nvd::fetch_cves_by_cpe(client, cpe, api_keys.get_nvd_api_key()).await
}

pub async fn fetch_feed_checksum(client: &Client) -> Result<String, Box<dyn Error>> {
    log::info!("fetching CPE match feed checksum ...");
    nvd::fetch_feed_checksum(client).await
}

pub fn get_cves_summary(
    full_cve_resp: &Value,
    known_exploitable_cves: Option<&[String]>,
) -> Vec<CveSummary> {
    nvd::get_cves_summary(full_cve_resp, known_exploitable_cves)
}

pub async fn download_cpe_match_feed(
    client: &Client,
    feed_path: &Path,
) -> Result<(), Box<dyn Error>> {
    log::info!("downloading CPE match feed ...");
    nvd::download_cpe_match_feed(client, feed_path).await
}

pub async fn fetch_known_exploited_vulns(client: &Client) -> Result<Value, Box<dyn Error>> {
    log::info!("fetching known exploited vulnerabilities ...");
    cisa::fetch_known_exploited_vulns(client).await
}

pub async fn fetch_known_exploited_cves(client: &Client) -> Result<Vec<String>, Box<dyn Error>> {
    log::info!("fetching known exploited CVEs ...");
    cisa::fetch_known_exploited_cves(client).await
}
