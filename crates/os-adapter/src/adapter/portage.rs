/*
 * SPDX-License-Identifier: MPL-2.0
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use package_model::package::{convert_to_pkg, Package};
use std::collections::HashMap;
use std::error::Error;
use std::fs::{read_dir, DirEntry};
use std::path::{Path, PathBuf};

pub trait Portage {
    fn get_pkg_dir(&self) -> &Path;
    fn set_pkg_dir(&mut self, pkg_dir: PathBuf);
    fn get_pkg_prefix_adapter(&self, category: &str) -> Option<&String>;
    fn set_pkg_adapter(&mut self, use_nvd_pkg_adapter: bool);
}

// https://riptutorial.com/rust/example/22917/inheritance-with-traits
impl<T> super::OsPackages for T
where
    T: Portage,
{
    fn set_pkg_dir(&mut self, pkg_dir: PathBuf) {
        self.set_pkg_dir(pkg_dir)
    }

    fn set_pkg_adapter(&mut self, use_nvd_pkg_adapter: bool) {
        self.set_pkg_adapter(use_nvd_pkg_adapter)
    }

    fn get_all_catpkgs(&self) -> Result<HashMap<String, Vec<Package>>, Box<dyn Error>> {
        let skipped_dirs = vec!["eclass", "licenses", "metadata", "profiles", "virtual"];
        let mut all_catpkgs = HashMap::new();

        if !&self.get_pkg_dir().exists() {
            log::error!("{:?} doesn't exist", &self.get_pkg_dir().as_os_str());
        } else {
            log::info!("walking {:?} ...", &self.get_pkg_dir().as_os_str());
        }

        for category in read_dir(&self.get_pkg_dir())? {
            let category = category?;
            let cat_path = &category.path();

            if !cat_path.is_dir() {
                continue;
            }

            match category.file_name().into_string() {
                Ok(ctgr) => {
                    if skipped_dirs.contains(&ctgr.as_str()) || ctgr.starts_with('.') {
                        log::debug!("SKIPPING packages in {} ...", ctgr);
                        continue;
                    }

                    log::debug!("collecting packages in {} ...", ctgr);
                    let pkgs = list_pkgs(cat_path, self.get_pkg_prefix_adapter(ctgr.as_str()))?;
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
        let pkg_path = &pkg.path();
        if !pkg_path.is_dir() {
            continue;
        }
        push_pkgs(pkg_path, prefix, &mut pkgs)?;
    }

    Ok(pkgs)
}

fn push_pkgs(
    path: &Path,
    prefix: Option<&String>,
    pkgs: &mut Vec<Package>,
) -> Result<(), Box<dyn Error>> {
    for entry in read_dir(path)? {
        let entry = entry?;
        if !is_ebuild(&entry) {
            continue;
        }

        if let Ok(ebuild) = entry.file_name().into_string() {
            let pkg: Vec<&str> = ebuild.rsplit(".ebuild").collect();
            let pkg = pkg[1].to_owned();
            if let Some(converted) = convert_to_pkg(&pkg) {
                pkgs.push(converted);
            }

            if let Some(prfx) = prefix {
                if let Some(converted) = convert_to_pkg(&format!("{}{}", prfx, &pkg)) {
                    pkgs.push(converted);
                }
            }
        }
    }

    Ok(())
}

fn is_ebuild(entry: &DirEntry) -> bool {
    if !entry.path().is_file() {
        return false;
    }

    if let Ok(file_name) = entry.file_name().into_string() {
        return file_name.ends_with(".ebuild");
    }

    log::error!("malformed file name {:?}", entry.file_name());
    false
}
