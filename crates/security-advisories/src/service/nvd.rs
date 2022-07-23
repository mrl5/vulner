/*
 * SPDX-License-Identifier: MPL-2.0
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::cve_summary::CveSummary;
use crate::utils::get_progress_bar;
use futures_util::StreamExt;
use reqwest::{Client, StatusCode};
use secrecy::{ExposeSecret, Secret};
use serde_json::Value;
use std::cmp::min;
use std::error::Error;
use std::io;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub const CPE_MATCH_FEED: &str = "nvdcpematch-1.0.json";
pub const CPE_MATCH_FEED_GZ: &str = "nvdcpematch-1.0.json.gz";
pub const CPE_KEYWORD_IN_FEED_LINE: &str = r#""cpe23Uri" : ""#;
const HOME_URL: &str = "https://nvd.nist.gov";
const API_URL: &str = "https://services.nvd.nist.gov/rest/json";
const CPE_FEED_PATH: &str = "feeds/json/cpematch/1.0";

pub async fn fetch_cves_by_cpe(
    client: &Client,
    cpe: &str,
    nvd_api_key: Option<Secret<String>>,
) -> Result<Value, Box<dyn Error>> {
    let cve_query_path = "cves/1.0";
    let cpe_query = "cpeMatchString";
    let mut url = format!("{}/{}?{}={}", API_URL, cve_query_path, cpe_query, cpe);

    if let Some(api_key) = nvd_api_key {
        url = format!("{}&apiKey={}", url, api_key.expose_secret());
    }

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::ACCEPT, "application/json".parse()?);
    let res = client.get(&url).headers(headers).send().await?;

    match res.error_for_status() {
        Ok(r) => {
            let json: Value = r.json().await?;
            Ok(json)
        }
        Err(err) => {
            if let Some(status) = err.status() {
                if status == StatusCode::FORBIDDEN {
                    log::error!(
                        "{} - {}: {}",
                        status,
                        "consider using NVD API key",
                        "https://nvd.nist.gov/developers/request-an-api-key"
                    );
                }
            }
            Err(Box::new(err))
        }
    }
}

pub fn get_cves_summary(
    full_cve_resp: &Value,
    known_exploitable_cves: Option<&[String]>,
) -> Result<Vec<CveSummary>, Box<dyn Error>> {
    let mut summary_items = vec![];

    if let Some(resp_items) = full_cve_resp["result"]["CVE_Items"].as_array() {
        for resp_item in resp_items {
            let cve_data = &resp_item["cve"];
            if let Some(summary) = get_cve_summary(cve_data, known_exploitable_cves) {
                summary_items.push(summary);
            }
        }
    }

    Ok(summary_items)
}

pub fn is_api_key_invalid(nvd_response: &Value) -> bool {
    log::debug!("{}", nvd_response);
    nvd_response["error"].as_str() == Some("Invalid apiKey")
}

pub async fn fetch_feed_checksum(client: &Client) -> Result<String, Box<dyn Error>> {
    let cpe_match_feed_meta = "nvdcpematch-1.0.meta";
    let url = format!("{}/{}/{}", HOME_URL, CPE_FEED_PATH, cpe_match_feed_meta);
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::ACCEPT, "text/plain".parse()?);
    let res = client.get(&url).headers(headers).send().await?;

    let meta = res.text().await?;
    Ok(get_checksum(meta)?)
}

pub async fn download_cpe_match_feed(
    client: &Client,
    feed_path: &Path,
) -> Result<(), Box<dyn Error>> {
    let url = format!("{}/{}/{}", HOME_URL, CPE_FEED_PATH, CPE_MATCH_FEED_GZ);
    let res = client.get(&url).send().await?;
    let total_size = res
        .content_length()
        .ok_or(format!("Failed to get content length from {}", url))?;
    log::info!("content length: {}", total_size);
    let pb = get_progress_bar(total_size, &url);

    let mut stream = res.bytes_stream();
    let mut downloaded: u64 = 0;
    let mut dest = {
        let f = feed_path.join(CPE_MATCH_FEED_GZ);
        File::create(f).await?
    };
    while let Some(item) = stream.next().await {
        let chunk = item?;
        dest.write_all(&chunk).await?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        pb.set_position(new);
    }

    pb.finish_with_message(format!("Downloaded {}", url));
    Ok(())
}

fn get_checksum(meta: String) -> Result<String, io::Error> {
    let keyword = "sha256:";
    let err_msg = r#"keyword not found: "{}""#;
    let err_kind = io::ErrorKind::InvalidData;

    let line = meta.lines().find(|x| {
        let l = x.to_owned();
        l.starts_with(keyword)
    });
    let line = match line {
        Some(v) => Ok(v),
        None => Err(io::Error::new(err_kind, err_msg)),
    };

    match line?.split(':').last() {
        Some(l) => Ok(l.to_ascii_lowercase()),
        None => Err(io::Error::new(err_kind, err_msg)),
    }
}

fn get_cve_summary(
    cve_data: &Value,
    known_exploitable_cves: Option<&[String]>,
) -> Option<CveSummary> {
    if let Some(id) = cve_data["CVE_data_meta"]["ID"].as_str() {
        let mut summary = CveSummary::new(
            id.to_owned(),
            get_cve_desc(cve_data),
            get_cve_urls(id, cve_data),
        );

        if let Some(kec) = known_exploitable_cves {
            summary.is_known_exploited_vuln = Some(kec.contains(&id.to_owned()));
        }

        return Some(summary);
    }
    None
}

fn get_cve_desc(cve_data: &Value) -> String {
    if let Some(descriptions) = cve_data["description"]["description_data"].as_array() {
        if let Some(desc) = descriptions.iter().find(|x| x["lang"] == "en") {
            if let Some(value) = desc["value"].as_str() {
                return value.to_owned();
            }
        }
    }
    "".to_owned()
}

fn get_cve_urls(id: &str, cve_data: &Value) -> Vec<String> {
    let nvd_url = "https://nvd.nist.gov/vuln/detail";
    let mut urls = vec![format!("{}/{}", nvd_url, id)];

    if let Some(ref_data) = cve_data["references"]["reference_data"].as_array() {
        for url in ref_data.iter().filter_map(|x| x["url"].as_str()) {
            urls.push(url.to_owned());
        }
    }

    urls
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn it_should_filter_checksum() {
        let checksum = "f283d332a8a66ecb23d49ee385ce42fca691e598da29475beb0b3556ab1fe02e";
        let test_data = "lastModifiedDate:2022-01-22T00:10:15-05:00\r\nsize:601269818\r\nzipSize:21871360\r\ngzSize:21871224\r\nsha256:F283D332A8A66ECB23D49EE385CE42FCA691E598DA29475BEB0B3556AB1FE02E\r\n";

        assert_eq!(get_checksum(test_data.to_owned()).unwrap(), checksum);
    }

    #[test]
    fn it_should_recognize_known_exploitable_cves() {
        let known_exploitable_cves = vec!["CVE-2021-22204".to_owned()];
        let listed_cve = from_str(r#"{"CVE_data_meta":{"ASSIGNER":"cve@gitlab.com","ID":"CVE-2021-22204"},"data_format":"MITRE","data_type":"CVE","data_version":"4.0","description":{"description_data":[{"lang":"en","value":"Improper neutralization of user data in the DjVu file format in ExifTool versions 7.44 and up allows arbitrary code execution when parsing the malicious image"}]},"problemtype":{"problemtype_data":[{"description":[{"lang":"en","value":"CWE-74"}]}]},"references":{"reference_data":[]}}"#).unwrap();
        let unlisted_cve = from_str(r#"{"CVE_data_meta":{"ASSIGNER":"cve@mitre.org","ID":"CVE-2022-23935"},"data_format":"MITRE","data_type":"CVE","data_version":"4.0","description":{"description_data":[{"lang":"en","value":"lib/Image/ExifTool.pm in ExifTool before 12.38 mishandles a $file =~ /\\|$/ check."}]},"problemtype":{"problemtype_data":[{"description":[{"lang":"en","value":"NVD-CWE-noinfo"}]}]},"references":{"reference_data":[]}}"#).unwrap();

        let summary = get_cve_summary(&listed_cve, None).unwrap();
        assert_eq!(summary.is_known_exploited_vuln, None);

        let summary = get_cve_summary(&listed_cve, Some(&known_exploitable_cves)).unwrap();
        assert_eq!(summary.is_known_exploited_vuln, Some(true));

        let summary = get_cve_summary(&unlisted_cve, Some(&known_exploitable_cves)).unwrap();
        assert_eq!(summary.is_known_exploited_vuln, Some(false));
    }

    #[test]
    fn it_should_recognize_invalid_api_key() {
        let cve = from_str(r#"{"CVE_data_meta":{"ASSIGNER":"cve@gitlab.com","ID":"CVE-2021-22204"},"data_format":"MITRE","data_type":"CVE","data_version":"4.0","description":{"description_data":[{"lang":"en","value":"Improper neutralization of user data in the DjVu file format in ExifTool versions 7.44 and up allows arbitrary code execution when parsing the malicious image"}]},"problemtype":{"problemtype_data":[{"description":[{"lang":"en","value":"CWE-74"}]}]},"references":{"reference_data":[]}}"#).unwrap();
        let invalid_api_key_resp = from_str(r#"{"loggedIn":true,"authorized":true,"error":"Invalid apiKey","message":"Invalid apiKey"}"#).unwrap();
        assert!(!is_api_key_invalid(&cve));
        assert!(is_api_key_invalid(&invalid_api_key_resp));
    }
}
