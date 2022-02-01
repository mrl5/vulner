/*
 * SPDX-License-Identifier: MPL-2.0
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::os_release::get_distro_id;
use cpe_tag::package::Package;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::io;
use std::str::FromStr;
use std::vec::Vec;
mod linux_funtoo;
mod linux_gentoo;
mod portage;

pub trait OsInfo {
    fn get_os(&self) -> &Os;
    fn get_os_flavor(&self) -> Option<OsFlavor>;
}

pub trait OsPackages {
    fn get_all_catpkgs(&self) -> Result<HashMap<String, Vec<Package>>, Box<dyn Error>>;
}

pub trait OsAdapter: OsInfo + OsPackages {}

pub fn get_adapter() -> Result<Box<dyn OsAdapter>, Box<dyn Error>> {
    let os = Os::from_str(env::consts::OS)?;

    if os == Os::GnuLinux {
        let id = LinuxDistro::from_str(&get_distro_id()?)?;
        Ok(get_linux_adapter(id))
    } else {
        // placeholder
        Err(Box::new(io::Error::from(io::ErrorKind::Unsupported)))
    }
}

#[derive(Eq, Debug, Hash, PartialEq)]
pub enum Os {
    GnuLinux,
}

#[derive(Clone, Debug, PartialEq)]
pub enum OsFlavor<'a> {
    LinuxDistro(&'a LinuxDistro),
}

#[derive(Debug, PartialEq)]
pub enum LinuxDistro {
    Funtoo,
    Gentoo,
}

pub fn get_supported_map() -> HashMap<Os, Vec<OsFlavor<'static>>> {
    HashMap::from([(
        Os::GnuLinux,
        vec![
            OsFlavor::LinuxDistro(&LinuxDistro::Funtoo),
            OsFlavor::LinuxDistro(&LinuxDistro::Gentoo),
        ],
    )])
}

fn get_linux_adapter(id: LinuxDistro) -> Box<dyn OsAdapter> {
    // https://refactoring.guru/design-patterns/factory-method
    match id {
        LinuxDistro::Funtoo => Box::new(linux_funtoo::Funtoo::new()),
        LinuxDistro::Gentoo => Box::new(linux_gentoo::Gentoo::new()),
    }
}

impl FromStr for Os {
    type Err = io::Error;

    fn from_str(input: &str) -> Result<Os, io::Error> {
        let unsupported_msg = "This OS is not supported";
        let err_kind = io::ErrorKind::Unsupported;
        let err_msg = format!("{}: {:?}", unsupported_msg, input);

        // https://doc.rust-lang.org/std/env/consts/constant.OS.html
        match input {
            "linux" => Ok(Os::GnuLinux),
            _ => Err(io::Error::new(err_kind, err_msg)),
        }
    }
}

impl FromStr for LinuxDistro {
    type Err = io::Error;

    fn from_str(input: &str) -> Result<LinuxDistro, io::Error> {
        let unsupported_msg = "This GNU/Linux distro is not supported";
        let err_kind = io::ErrorKind::Unsupported;
        let err_msg = format!("{}: {:?}", unsupported_msg, input);

        // ID of https://www.freedesktop.org/software/systemd/man/os-release.html
        match input {
            "funtoo" => Ok(LinuxDistro::Funtoo),
            "gentoo" => Ok(LinuxDistro::Gentoo),
            _ => Err(io::Error::new(err_kind, err_msg)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_know_supported_oses() {
        assert!(Os::from_str("linux").is_ok());
        assert!(Os::from_str("freebsd").is_err());
    }

    #[test]
    fn it_should_know_supported_distros() {
        assert!(LinuxDistro::from_str("funtoo").is_ok());
        assert!(LinuxDistro::from_str("gentoo").is_ok());
        assert!(LinuxDistro::from_str("debian").is_err());
    }
}
