/*
 * SPDX-License-Identifier: MPL-2.0
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::string::ToString;

pub fn convert_to_pkg(raw_pkg: &str) -> Option<Package> {
    let pattern = "(.+)-([0-9]+.*)";
    let re = Regex::new(pattern).unwrap();
    let caps = re.captures(raw_pkg)?;

    let name = caps.get(1);
    let version = caps.get(2);

    if name.is_some() && version.is_some() {
        let name = name?.as_str().to_owned();
        let version = version?.as_str().to_owned();
        Some(Package::new(name, version))
    } else {
        None
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct Package {
    pub name: String,
    pub version: String,
}

impl Package {
    pub fn new(name: String, version: String) -> Self {
        Self { name, version }
    }
}

impl ToString for Package {
    fn to_string(&self) -> String {
        format!("{}-{}", self.name, self.version)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_convert_raw_pkg_into_struct() {
        let raw_pkg = "rust-bin-1.58.1";
        let expected = Some(Package::new("rust-bin".to_owned(), "1.58.1".to_owned()));
        assert_eq!(expected, convert_to_pkg(raw_pkg));
    }

    #[test]
    fn it_should_implement_to_string() {
        let expected = "rust-bin-1.58.1";
        let package = Package::new("rust-bin".to_owned(), "1.58.1".to_owned());
        assert_eq!(expected.to_owned(), package.to_string());
    }
}
