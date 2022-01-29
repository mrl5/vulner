/*
 * SPDX-License-Identifier: MPL-2.0
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use cpe_tag::validators::validate_cpe_batch;
use security_advisories::http::get_client;
use security_advisories::service::fetch_cves_by_cpe;
use serde_json::from_str;
use std::error::Error;

pub async fn execute(batch: String) -> Result<(), Box<dyn Error>> {
    log::info!("validating input ...");
    let json = from_str(&batch)?;
    validate_cpe_batch(&json)?;

    let client = get_client()?;
    for v in json.as_array().unwrap_or(&vec![]) {
        match v.as_str() {
            Some(cpe) => {
                let res = fetch_cves_by_cpe(&client, cpe).await?;
                println!("{}", res);
            }
            None => continue,
        }
    }

    Ok(())
}
