/*
 * SPDX-License-Identifier: MPL-2.0
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use super::portage::Portage;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct Funtoo {
    os: super::Os,
    flavor: super::LinuxDistro,
    pkg_dir: PathBuf,
    pkg_prefix_adapter: HashMap<&'static str, String>,
}

impl Funtoo {
    pub fn new() -> Self {
        Self {
            os: super::Os::GnuLinux,
            flavor: super::LinuxDistro::Funtoo,
            pkg_dir: Path::new("/var/db/pkg/").to_path_buf(),
            pkg_prefix_adapter: HashMap::from([]),
        }
    }
}

impl super::OsAdapter for Funtoo {}

impl super::OsInfo for Funtoo {
    fn get_os(&self) -> &super::Os {
        &self.os
    }

    fn get_os_flavor(&self) -> Option<super::OsFlavor> {
        Some(super::OsFlavor::LinuxDistro(&self.flavor))
    }
}

impl Portage for Funtoo {
    fn get_pkg_dir(&self) -> &Path {
        &self.pkg_dir
    }

    fn set_pkg_dir(&mut self, pkg_dir: PathBuf) {
        self.pkg_dir = pkg_dir;
    }

    fn get_pkg_prefix_adapter(&self, category: &str) -> Option<&String> {
        self.pkg_prefix_adapter.get(category)
    }

    fn set_pkg_adapter(&mut self, use_nvd_pkg_adapter: bool) {
        if use_nvd_pkg_adapter {
            self.pkg_prefix_adapter = HashMap::from([("dev-libs", "lib".to_owned())])
        }
    }
}
