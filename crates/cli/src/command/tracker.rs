/*
 * SPDX-License-Identifier: MPL-2.0
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use os_adapter::adapter::get_adapter;
use security_advisories::http::get_client;
use security_advisories::service::get_distro_tracker_summary;
use serde_json::to_string;
use std::error::Error;

pub async fn execute() -> Result<(), Box<dyn Error>> {
    let os = get_adapter(None, None)?;
    let client = get_client()?;
    let tracker_summary = get_distro_tracker_summary(&client, os).await?;

    println!("{}", to_string(&tracker_summary)?);
    Ok(())
}
