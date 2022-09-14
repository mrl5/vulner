/*
 * SPDX-License-Identifier: MPL-2.0
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use reqwest::Client;
use std::error::Error;
use std::time::Duration;

pub fn get_client(custom_user_agent: Option<String>) -> Result<Client, Box<dyn Error>> {
    let is_verbose = true;
    let tcp_keepalive = Duration::from_secs(180);
    let use_gzip = true;

    let mut client = Client::builder()
        .connection_verbose(is_verbose)
        .tcp_keepalive(tcp_keepalive)
        .gzip(use_gzip);

    if let Some(user_agent) = custom_user_agent {
        client = client.user_agent(user_agent);
    }

    let client = client.build()?;
    Ok(client)
}
