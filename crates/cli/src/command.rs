/*
 * SPDX-License-Identifier: MPL-2.0
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::input::get_input;
use std::error::Error;
use std::path::PathBuf;
use structopt::StructOpt;
mod cpe;
mod cve;
mod scan;
mod sync;

pub async fn execute(cmd: Command) -> Result<(), Box<dyn Error>> {
    match cmd {
        Command::Sync { cpe_feed } => sync::execute(cpe_feed.feed_dir).await,
        Command::Cpe {
            packages_batch,
            cpe_feed,
        } => cpe::execute(get_input(packages_batch)?, cpe_feed.feed_dir).await,
        Command::Cve { cpe_batch, summary } => cve::execute(get_input(cpe_batch)?, summary).await,
        Command::Scan { cpe_feed, out_dir } => scan::execute(cpe_feed.feed_dir, out_dir).await,
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
        packages_batch: Option<String>,
        #[structopt(flatten)]
        cpe_feed: CpeFeedOpt,
    },

    #[structopt(name = "cve", about = "Lists CVEs for given CPEs")]
    Cve {
        cpe_batch: Option<String>,
        #[structopt(short, long, help = "Prints CVE summary instead of full response")]
        summary: bool,
    },

    #[structopt(
        name = "scan",
        about = "Scans for CVEs in software installed by the OS package manager"
    )]
    Scan {
        #[structopt(flatten)]
        cpe_feed: CpeFeedOpt,

        #[structopt(short = "o", long = "out-dir", env = "VULNER_OUT_DIR")]
        out_dir: PathBuf,
    },
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
