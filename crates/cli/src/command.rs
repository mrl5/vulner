/*
 * SPDX-License-Identifier: MPL-2.0
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::error::Error;
use std::path::PathBuf;
use structopt::StructOpt;
mod sync;

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(name = "sync", about = "Synchronizes CPE match feed")]
    Sync {
        #[structopt(
            short = "d",
            long = "feed-dir",
            default_value = "/tmp/vulner/feeds/json",
            env = "VULNER_FEED_DIR"
        )]
        feed_dir: PathBuf,
    },
    // todo: scan
    // todo: cpe
}

pub async fn execute(cmd: Command) -> Result<(), Box<dyn Error>> {
    match cmd {
        Command::Sync { feed_dir } => sync::execute(feed_dir).await,
    }
}
