/*
 * SPDX-License-Identifier: MPL-2.0
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use secrecy::Secret;
use serde::{Deserialize, Serialize};
use std::env;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(Serialize, Deserialize, Debug)]
pub struct VulnerConfig {
    version: u8,
    pub scan_results_dir: PathBuf,
    pub api_keys: ApiKeys,
}

#[derive(Serialize, Deserialize, Debug, StructOpt)]
pub struct ApiKeys {
    #[structopt(env = "NVD_API_KEY")]
    nvd_api_key: Option<String>,
}

impl std::default::Default for VulnerConfig {
    fn default() -> Self {
        let home_dir = env::var("HOME").unwrap_or_else(|_| "/tmp".to_owned());
        let vulner_dir = Path::new(&home_dir).join(crate::NAME);

        Self {
            version: 0,
            scan_results_dir: vulner_dir.join("scan-results"),
            api_keys: ApiKeys::default(),
        }
    }
}

impl std::default::Default for ApiKeys {
    fn default() -> Self {
        Self {
            nvd_api_key: Some("".to_owned()),
        }
    }
}

impl security_advisories::service::ApiKeys for ApiKeys {
    fn get_nvd_api_key(&self) -> Option<Secret<String>> {
        if let Ok(nvd_api_key) = env::var("NVD_API_KEY") {
            if !nvd_api_key.is_empty() {
                return Some(Secret::new(nvd_api_key));
            }
        }

        if let Some(nvd_api_key) = &self.nvd_api_key {
            if !nvd_api_key.is_empty() {
                return Some(Secret::new(nvd_api_key.to_owned()));
            }
        }

        None
    }
}
