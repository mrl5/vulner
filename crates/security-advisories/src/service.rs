/*
 * SPDX-License-Identifier: MPL-2.0
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::cve_summary::CveSummary;
use crate::tracker_summary::DistroTrackerSummary;
use os_adapter::adapter::{LinuxDistro, Os, OsAdapter, OsFlavor};
use reqwest::Client;
use secrecy::Secret;
use serde_json::Value;
use std::error::Error;
use std::io::{Error as IOError, ErrorKind};
use std::path::Path;
mod cisa;
mod funtoo;
mod nvd;

pub const CPE_MATCH_FEED: &str = nvd::CPE_MATCH_FEED;
pub const CPE_MATCH_FEED_GZ: &str = nvd::CPE_MATCH_FEED_GZ;
pub const CPE_KEYWORD_IN_FEED_LINE: &str = nvd::CPE_KEYWORD_IN_FEED_LINE;

pub trait ApiKeys {
    fn get_nvd_api_key(&self) -> Option<Secret<String>>;
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
) -> Result<Vec<CveSummary>, Box<dyn Error>> {
    throw_on_invalid_api_key(full_cve_resp)?;
    nvd::get_cves_summary(full_cve_resp, known_exploitable_cves)
}

pub fn throw_on_invalid_api_key(resp: &Value) -> Result<(), Box<dyn Error>> {
    if nvd::is_api_key_invalid(resp) {
        return Err(Box::new(IOError::new(
            ErrorKind::InvalidInput,
            "Invalid NVD API key",
        )));
    }
    Ok(())
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

pub async fn get_distro_tracker_summary(
    client: &Client,
    os: &'_ dyn OsAdapter,
) -> Result<Vec<DistroTrackerSummary>, Box<dyn Error>> {
    if *os.get_os() != Os::GnuLinux {
        return Err(Box::new(IOError::from(ErrorKind::Unsupported)));
    }

    match os.get_os_flavor() {
        Some(OsFlavor::LinuxDistro(&LinuxDistro::Funtoo)) => funtoo::get_vuln_tracker(client).await,
        _ => Err(Box::new(IOError::from(ErrorKind::Unsupported))),
    }
}

pub async fn get_distro_tickets_by_cve(
    client: &Client,
    os: &'_ dyn OsAdapter,
    cve_id: String,
) -> Result<Vec<String>, Box<dyn Error>> {
    if *os.get_os() != Os::GnuLinux {
        return Err(Box::new(IOError::from(ErrorKind::Unsupported)));
    }

    match os.get_os_flavor() {
        Some(OsFlavor::LinuxDistro(&LinuxDistro::Funtoo)) => {
            funtoo::get_tickets_by_cve(client, cve_id).await
        }
        _ => Err(Box::new(IOError::from(ErrorKind::Unsupported))),
    }
}
