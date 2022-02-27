/*
 * SPDX-License-Identifier: MPL-2.0
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use reqwest::Client;
use serde_json::Value;
use std::error::Error;

pub async fn fetch_known_exploited_vulns(client: &Client) -> Result<Value, Box<dyn Error>> {
    let home_url = "https://www.cisa.gov";
    let feed_path = "sites/default/files/feeds";
    let known_exploited_vulns = "known_exploited_vulnerabilities.json";
    let url = format!("{}/{}/{}", home_url, feed_path, known_exploited_vulns);
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::ACCEPT, "application/json".parse()?);

    log::info!("source: {}", home_url);
    let res = client.get(&url).headers(headers).send().await?;

    let json: Value = res.json().await?;
    Ok(json)
}

pub async fn fetch_known_exploited_cves(client: &Client) -> Result<Vec<String>, Box<dyn Error>> {
    let known_exploited_vulns = fetch_known_exploited_vulns(client).await?;

    Ok(get_known_exploited_cves(&known_exploited_vulns))
}

fn get_known_exploited_cves(known_exploited_vulns: &Value) -> Vec<String> {
    let mut cves = vec![];
    if let Some(kev) = known_exploited_vulns["vulnerabilities"].as_array() {
        for item in kev {
            if let Some(cve) = item["cveID"].as_str() {
                cves.push(cve.to_owned());
            }
        }
    } else {
        log::error!("couldn't get known exploited CVEs");
    }

    cves
}
