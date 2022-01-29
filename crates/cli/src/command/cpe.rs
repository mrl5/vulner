/*
 * SPDX-License-Identifier: MPL-2.0
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use cpe_tag::query_builder::{get_grep_patterns, query};
use cpe_tag::validators::validate_packages_batch;
use security_advisories::service::CPE_MATCH_FEED;
use std::error::Error;
use std::path::PathBuf;

pub async fn execute(batch: String, feed_dir: PathBuf) -> Result<(), Box<dyn Error>> {
    log::info!("validating input ...");
    validate_packages_batch(batch.as_ref())?;

    log::info!("building query ...");
    let pattern = get_grep_patterns(batch.as_ref())?;
    log::info!("searching patterns in CPE match feed ...");
    let matches = query(pattern, &feed_dir.join(CPE_MATCH_FEED))?;

    println!("{:?}", matches);
    Ok(())
}
