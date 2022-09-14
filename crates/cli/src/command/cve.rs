/*
 * SPDX-License-Identifier: MPL-2.0
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::conf::ApiKeys;
use cpe_tag::validators::validate_cpe_batch;
use security_advisories::http::get_client;
use security_advisories::service::{
    fetch_cves_by_cpe, fetch_known_exploited_cves, get_cves_summary, throw_on_invalid_api_key,
};
use serde_json::from_str;
use serde_json::Value;
use std::error::Error;

pub async fn execute(
    batch: String,
    show_summary: bool,
    check_known_exploited: bool,
    api_keys: ApiKeys,
) -> Result<(), Box<dyn Error>> {
    log::info!("validating input ...");
    let json = from_str(&batch)?;
    validate_cpe_batch(&json)?;

    let client = get_client(Some(format!("vulner {}", env!("CARGO_PKG_VERSION"))))?;
    let mut feed = vec![];
    if check_known_exploited {
        feed = fetch_known_exploited_cves(&client).await?;
    }

    for v in json.as_array().unwrap_or(&vec![]) {
        match v.as_str() {
            Some(cpe) => {
                log::info!("fetching CVEs by CPE {} ...", cpe);
                let cves = fetch_cves_by_cpe(&client, cpe, &api_keys).await?;
                print_cves(
                    cves,
                    show_summary,
                    if check_known_exploited {
                        Some(&feed)
                    } else {
                        None
                    },
                )?;
            }
            None => continue,
        }
    }

    Ok(())
}

fn print_cves(
    cves: Value,
    show_summary: bool,
    known_exploited_cves: Option<&[String]>,
) -> Result<(), Box<dyn Error>> {
    if show_summary {
        match get_cves_summary(&cves, known_exploited_cves) {
            Ok(summary) => {
                for cve in summary {
                    println!("{}", cve);
                }
                Ok(())
            }
            Err(e) => Err(e),
        }
    } else {
        throw_on_invalid_api_key(&cves)?;
        println!("{}", cves);
        Ok(())
    }
}
