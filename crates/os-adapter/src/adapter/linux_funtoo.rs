/*
 * SPDX-License-Identifier: MPL-2.0
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use cpe_tag::package::{convert_to_pkg, Package};
use std::collections::HashMap;
use std::error::Error;
use std::fs::read_dir;
use std::path::Path;

pub struct Funtoo {
    os: super::Os,
    flavor: super::LinuxDistro,
    pkg_dir: String,
}

impl Funtoo {
    pub fn new() -> Self {
        Self {
            os: super::Os::GnuLinux,
            flavor: super::LinuxDistro::Funtoo,
            pkg_dir: "/var/db/pkg/".to_owned(),
        }
    }
}

impl super::OsAdapter for Funtoo {
    fn get_os(&self) -> &super::Os {
        &self.os
    }

    fn get_os_flavor(&self) -> Option<super::OsFlavor> {
        Some(super::OsFlavor::LinuxDistro(&self.flavor))
    }

    fn get_all_catpkgs(&self) -> Result<HashMap<String, Vec<Package>>, Box<dyn Error>> {
        let pkg_prefix_adapter: HashMap<&str, String> =
            HashMap::from([("dev-libs", "lib".to_owned())]);
        let skipped_dirs = vec!["virtual"];
        let mut all_catpkgs = HashMap::new();

        log::info!("walking {} ...", &self.pkg_dir);
        for category in read_dir(Path::new(&self.pkg_dir))? {
            let category = category?;
            let path = &category.path();

            if !path.is_dir() {
                continue;
            }

            match category.file_name().into_string() {
                Ok(ctgr) => {
                    if skipped_dirs.contains(&ctgr.as_str()) {
                        log::warn!("SKIPPING packages in {} ...", ctgr);
                        continue;
                    }

                    log::debug!("collecting packages in {} ...", ctgr);
                    let pkgs = list_pkgs(path, pkg_prefix_adapter.get(ctgr.as_str()))?;
                    all_catpkgs.insert(ctgr, pkgs);
                }
                Err(os_path) => {
                    log::error!("skipping {:?}", os_path);
                    continue;
                }
            }
        }

        Ok(all_catpkgs)
    }
}

fn list_pkgs(path: &Path, prefix: Option<&String>) -> Result<Vec<Package>, Box<dyn Error>> {
    let mut pkgs: Vec<Package> = vec![];

    for pkg in read_dir(path)? {
        let pkg = pkg?;
        let path = &pkg.path();

        if !path.is_dir() {
            continue;
        }

        match pkg.file_name().into_string() {
            Ok(p) => {
                if let Some(converted) = convert_to_pkg(&p) {
                    pkgs.push(converted);
                }

                if let Some(prfx) = prefix {
                    if let Some(converted) = convert_to_pkg(&format!("{}{}", prfx, &p)) {
                        pkgs.push(converted);
                    }
                }
            }
            Err(os_string) => {
                log::error!("skipping {:?}", os_string);
                continue;
            }
        }
    }

    Ok(pkgs)
}
