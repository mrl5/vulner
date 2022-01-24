/*
 * SPDX-License-Identifier: MPL-2.0
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::env;
use std::process::exit;
use structopt::{clap::AppSettings, StructOpt};
mod command;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "vulner",
    about = env!("CARGO_PKG_DESCRIPTION"),
    global_settings(&[
      AppSettings::ColoredHelp
    ]),
)]
struct CliOptions {
    #[structopt(subcommand)]
    cmd: command::Command,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    log::debug!("initialized logger");
    let opts = CliOptions::from_args();

    exit(match command::execute(opts.cmd).await {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("error: {}", e);
            1
        }
    });
}
