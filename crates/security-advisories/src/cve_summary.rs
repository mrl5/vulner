/*
 * SPDX-License-Identifier: MPL-2.0
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use serde::Serialize;
use serde_json::to_string;
use std::fmt;

#[derive(Serialize, Debug)]
pub struct CveSummary {
    pub id: String,
    pub description: String,
    pub urls: Vec<String>,
}

impl CveSummary {
    pub fn new(id: String, description: String, urls: Vec<String>) -> Self {
        Self {
            id,
            description,
            urls,
        }
    }
}

impl fmt::Display for CveSummary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            to_string(&self).unwrap_or_else(|_| "{}".to_owned())
        )
    }
}
