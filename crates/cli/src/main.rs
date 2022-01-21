/*
 * SPDX-License-Identifier: MPL-2.0
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use structopt::{clap::AppSettings, StructOpt};

#[derive(StructOpt)]
#[structopt(
    name = "vulner",
    about = env!("CARGO_PKG_DESCRIPTION"),
    global_settings(&[
      AppSettings::ColoredHelp
    ]),
)]
struct CliOptions {}

fn main() {
    env_logger::init();
    log::debug!("initialized logger");
    let options = CliOptions::from_args();
    println!("Hello, world!");
}
