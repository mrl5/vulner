/*
 * SPDX-License-Identifier: MPL-2.0
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::conf;
use crate::input::get_input;
use confy::load;
use std::error::Error;
use std::path::PathBuf;
use structopt::StructOpt;
mod cpe;
mod cve;
mod known_exploited_vulns;
mod scan;
mod sync;
mod tracker;

pub async fn execute(cmd: Command) -> Result<(), Box<dyn Error>> {
    let cfg: conf::VulnerConfig = load(env!("CARGO_BIN_NAME"), "vulner").unwrap_or_default();
    log::debug!("loaded cfg {:#?}", cfg);

    match cmd {
        Command::Sync { cpe_feed } => sync::execute(cpe_feed.feed_dir).await,

        Command::Cpe {
            packages_batch,
            cpe_feed,
        } => cpe::execute(get_input(packages_batch)?, cpe_feed.feed_dir).await,

        Command::Cve {
            cpe_batch,
            summary,
            check_known_exploited,
            api_keys: _,
        } => {
            cve::execute(
                get_input(cpe_batch)?,
                summary,
                check_known_exploited,
                cfg.api_keys,
            )
            .await
        }

        Command::Scan {
            cpe_feed,
            out_dir,
            pkg_dir,
            recursive,
            api_keys: _,
            no_bugtracker,
        } => {
            scan::execute(
                cpe_feed.feed_dir,
                out_dir.unwrap_or(cfg.scan_results_dir),
                pkg_dir,
                recursive,
                cfg.api_keys,
                no_bugtracker,
            )
            .await
        }

        Command::KnownExploitedVulns {} => known_exploited_vulns::execute().await,

        Command::Tracker {} => tracker::execute().await,
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

        #[structopt(
            short,
            long,
            help = "Additonal check agains known exploited vulnerabilities catalog"
        )]
        check_known_exploited: bool,

        #[structopt(flatten)]
        api_keys: conf::ApiKeys,
    },

    #[structopt(
        name = "scan",
        about = "Scans for CVEs in software installed by the OS package manager"
    )]
    Scan {
        #[structopt(flatten)]
        cpe_feed: CpeFeedOpt,

        #[structopt(short = "o", long = "out-dir", env = "VULNER_OUT_DIR")]
        out_dir: Option<PathBuf>,

        #[structopt(short = "p", long = "pkg-dir", env = "VULNER_PKG_DIR")]
        pkg_dir: Option<PathBuf>,

        #[structopt(
            short,
            long,
            help = "Recurisve scan for Funtoo Linux meta-repo",
            required_if("pkg-dir", "/var/git/meta-repo"),
            required_if("pkg-dir", "/var/git/meta-repo/")
        )]
        recursive: bool,

        #[structopt(flatten)]
        api_keys: conf::ApiKeys,

        #[structopt(short, long, help = "Don't query distro bugtracker")]
        no_bugtracker: bool,
    },

    #[structopt(
        name = "kev",
        about = "Prints (K)nown (E)xploited (V)ulnerabilities catalog"
    )]
    KnownExploitedVulns {},

    #[structopt(
        name = "tracker",
        about = "Prints contents of OS vulnerability tracker"
    )]
    Tracker {},
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
