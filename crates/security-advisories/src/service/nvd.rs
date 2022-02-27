/*
 * SPDX-License-Identifier: MPL-2.0
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::cve_summary::CveSummary;
use crate::utils::get_progress_bar;
use futures_util::StreamExt;
use reqwest::Client;
use serde_json::Value;
use std::cmp::min;
use std::error::Error;
use std::io;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub const CPE_MATCH_FEED: &str = "nvdcpematch-1.0.json";
pub const CPE_MATCH_FEED_GZ: &str = "nvdcpematch-1.0.json.gz";
const HOME_URL: &str = "https://nvd.nist.gov";
const API_URL: &str = "https://services.nvd.nist.gov/rest/json";
const CPE_FEED_PATH: &str = "feeds/json/cpematch/1.0";

pub async fn fetch_cves_by_cpe(client: &Client, cpe: &str) -> Result<Value, Box<dyn Error>> {
    let cve_query_path = "cves/1.0";
    let cpe_query = "cpeMatchString";
    let url = format!("{}/{}?{}={}", API_URL, cve_query_path, cpe_query, cpe);
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::ACCEPT, "application/json".parse()?);
    let res = client.get(&url).headers(headers).send().await?;

    let json: Value = res.json().await?;
    Ok(json)
}

pub fn get_cves_summary(full_cve_resp: &Value) -> Vec<CveSummary> {
    let mut ids = vec![];
    if let Some(items) = full_cve_resp["result"]["CVE_Items"].as_array() {
        for item in items {
            let cve_data = &item["cve"];
            if let Some(summary) = get_cve_summary(cve_data) {
                ids.push(summary);
            }
        }
    }

    ids
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

fn get_cve_summary(cve_data: &Value) -> Option<CveSummary> {
    if let Some(id) = cve_data["CVE_data_meta"]["ID"].as_str() {
        return Some(CveSummary::new(
            id.to_owned(),
            get_cve_desc(cve_data),
            get_cve_urls(id, cve_data),
        ));
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
    #[test]
    fn it_should_filter_checksum() {
        let checksum = "f283d332a8a66ecb23d49ee385ce42fca691e598da29475beb0b3556ab1fe02e";
        let test_data = "lastModifiedDate:2022-01-22T00:10:15-05:00\r\nsize:601269818\r\nzipSize:21871360\r\ngzSize:21871224\r\nsha256:F283D332A8A66ECB23D49EE385CE42FCA691E598DA29475BEB0B3556AB1FE02E\r\n";

        assert_eq!(get_checksum(test_data.to_owned()).unwrap(), checksum);
    }
}
