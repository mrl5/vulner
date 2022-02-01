/*
 * SPDX-License-Identifier: MPL-2.0
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use super::portage::Portage;
use std::path::Path;

pub struct Funtoo {
    os: super::Os,
    flavor: super::LinuxDistro,
}

impl Funtoo {
    pub fn new() -> Self {
        Self {
            os: super::Os::GnuLinux,
            flavor: super::LinuxDistro::Funtoo,
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
        Path::new("/var/db/pkg/")
    }
}
