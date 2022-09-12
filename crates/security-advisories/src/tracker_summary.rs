/*
 * SPDX-License-Identifier: MPL-2.0
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use serde::Serialize;
use serde_json::to_string;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Serialize, Debug)]
pub struct DistroTrackerSummary {
    pub id: String,
    pub url: String,
    pub summary: String,
}

impl DistroTrackerSummary {
    pub fn new(id: String, url: String, summary: String) -> Self {
        Self { id, url, summary }
    }
}

impl fmt::Display for DistroTrackerSummary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            to_string(&self).unwrap_or_else(|_| "{}".to_owned())
        )
    }
}

impl Hash for DistroTrackerSummary {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for DistroTrackerSummary {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for DistroTrackerSummary {}
