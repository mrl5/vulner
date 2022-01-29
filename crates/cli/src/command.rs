/*
 * SPDX-License-Identifier: MPL-2.0
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::error::Error;
use std::path::PathBuf;
use structopt::StructOpt;
mod cpe;
mod sync;

pub async fn execute(cmd: Command) -> Result<(), Box<dyn Error>> {
    match cmd {
        Command::Sync { cpe_feed } => sync::execute(cpe_feed.feed_dir).await,
        Command::Cpe {
            packages_batch,
            cpe_feed,
        } => cpe::execute(packages_batch, cpe_feed.feed_dir).await,
    }
}

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(name = "sync", about = "Synchronizes CPE match feed")]
    Sync {
        #[structopt(flatten)]
        cpe_feed: CpeFeedOpt,
    },

    #[structopt(name = "cpe", about = "Provides valid and existing CPEs")]
    Cpe {
        packages_batch: String,
        #[structopt(flatten)]
        cpe_feed: CpeFeedOpt,
    },
    // todo: scan
}

#[derive(Debug, StructOpt)]
pub struct CpeFeedOpt {
    #[structopt(
        short = "d",
        long = "feed-dir",
        default_value = "/tmp/vulner/feeds/json",
        env = "VULNER_FEED_DIR"
    )]
    feed_dir: PathBuf,
}
