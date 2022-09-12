/*
 * SPDX-License-Identifier: MPL-2.0
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::tracker_summary::DistroTrackerSummary;
use reqwest::{Client, Url};
use serde_json::Value;
use std::error::Error;

const BUGS_URL: &str = "https://bugs.funtoo.org";
const UI_PATH: &str = "browse";
const API_PATH: &str = "rest/api/latest";
const VULN_BUG_TYPE: &str = "10200";

pub async fn get_vuln_tracker(
    client: &Client,
) -> Result<Vec<DistroTrackerSummary>, Box<dyn Error>> {
    let resp = fetch_vuln_tracker(client).await?;
    let mut summary = vec![];

    if let Some(issues) = resp["issues"].as_array() {
        for issue in issues {
            let id = issue["key"].as_str().unwrap_or("");
            let url = format!("{}/{}/{}", BUGS_URL, UI_PATH, id);
            let issue_summary = issue["fields"]["summary"].as_str().unwrap_or("");

            summary.push(DistroTrackerSummary::new(
                id.to_owned(),
                url.to_string(),
                issue_summary.to_owned(),
            ))
        }
    }

    Ok(summary)
}

async fn fetch_vuln_tracker(client: &Client) -> Result<Value, Box<dyn Error>> {
    let query = format!("issuetype = {} AND statuscategory != Done", VULN_BUG_TYPE);
    let url = format!(
        "{}/{}/search?fields=key,summary&jql={}",
        BUGS_URL, API_PATH, query
    );
    let url = Url::parse(&url)?;
    let res = client.get(url.to_string()).send().await?;
    let json: Value = res.json().await?;

    Ok(json)
}
