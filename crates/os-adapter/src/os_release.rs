/*
 * SPDX-License-Identifier: MPL-2.0
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// https://www.freedesktop.org/software/systemd/man/os-release.html

pub fn get_distro_id() -> Result<String, Box<dyn Error>> {
    let file = File::open(get_os_release_path())?;
    let kw = "ID=";
    let id = io::BufReader::new(file)
        .lines()
        .find(|x| match x {
            Ok(v) => v.starts_with(kw),
            Err(_) => false,
        })
        .unwrap_or_else(|| Err(io::Error::from(io::ErrorKind::Unsupported)))?;

    let id = id
        .rsplit(kw)
        .collect::<String>()
        .trim_matches('"')
        .to_owned();
    log::info!("detected distro: {}", &id);
    Ok(id)
}

fn get_os_release_path() -> &'static Path {
    // The file /etc/os-release takes precedence over /usr/lib/os-release

    let etc = Path::new("/etc/os-release");
    let usr_bin = Path::new("/usr/lib/os-release");

    if etc.exists() {
        log::debug!("using {:?}", etc);
        return etc;
    }
    log::debug!("fallback to {:?}", usr_bin);
    usr_bin
}
