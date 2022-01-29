/*
 * SPDX-License-Identifier: MPL-2.0
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use regex::Regex;
use serde::{Deserialize, Serialize};

pub fn convert_to_pkg(raw_pkg: &str) -> Option<Package> {
    let pattern = "(.+)-([0-9]+.*)";
    let re = Regex::new(pattern).unwrap();
    let caps = re.captures(raw_pkg)?;

    let name = caps.get(1);
    let version = caps.get(2);

    if name.is_some() && version.is_some() {
        let name = name?.as_str().to_owned();
        let version = version?.as_str().to_owned();
        Some(Package::new(name, vec![Version::new(version)]))
    } else {
        None
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Package {
    name: String,
    versions: Vec<Version>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Version {
    version: String,
}

impl Package {
    pub fn new(name: String, versions: Vec<Version>) -> Self {
        Self { name, versions }
    }
}

impl Version {
    pub fn new(version: String) -> Self {
        Self { version }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_convert_raw_pkg_into_struct() {
        let raw_pkg = "rust-bin-1.58.1";
        let expected = Some(Package::new(
            "rust-bin".to_owned(),
            vec![Version::new("1.58.1".to_owned())],
        ));
        assert_eq!(expected, convert_to_pkg(raw_pkg));
    }
}
