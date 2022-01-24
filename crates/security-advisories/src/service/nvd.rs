/*
 * SPDX-License-Identifier: MPL-2.0
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::utils::get_progress_bar;
use futures_util::StreamExt;
use reqwest::Client;
use std::cmp::min;
use std::error::Error;
use std::io;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

const BASE_URL: &str = "https://nvd.nist.gov/feeds/json/cpematch/1.0";
pub const CPE_MATCH_FEED: &str = "nvdcpematch-1.0.json.gz";
const CPE_MATCH_FEED_META: &str = "nvdcpematch-1.0.meta";

pub async fn fetch_feed_checksum(client: &Client) -> Result<String, Box<dyn Error>> {
    let url = format!("{}/{}", BASE_URL, CPE_MATCH_FEED_META);
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
    let url = format!("{}/{}", BASE_URL, CPE_MATCH_FEED);
    let res = client.get(&url).send().await?;
    let total_size = res
        .content_length()
        .ok_or(format!("Failed to get content length from {}", url))?;
    let pb = get_progress_bar(total_size, &url);

    let mut stream = res.bytes_stream();
    let mut downloaded: u64 = 0;
    let mut dest = {
        let f = feed_path.join(CPE_MATCH_FEED);
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
