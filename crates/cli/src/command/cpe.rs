/*
 * SPDX-License-Identifier: MPL-2.0
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use cpe_tag::query_builder::{get_grep_patterns, query};
use cpe_tag::validators::into_validated_packages;
use security_advisories::service::CPE_MATCH_FEED;
use serde_json::from_str;
use std::error::Error;
use std::path::PathBuf;

pub async fn execute(batch: String, feed_dir: PathBuf) -> Result<(), Box<dyn Error>> {
    log::info!("validating input ...");
    let json = from_str(&batch)?;
    let json = into_validated_packages(&json)?;

    log::info!("building query ...");
    let pattern = get_grep_patterns(&json)?;
    log::info!("searching patterns in CPE match feed ...");
    let matches = query(pattern, &feed_dir.join(CPE_MATCH_FEED))?;

    println!("{:?}", matches);
    Ok(())
}
