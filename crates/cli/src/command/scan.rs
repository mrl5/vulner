/*
 * SPDX-License-Identifier: MPL-2.0
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use chrono::{Timelike, Utc};
use cpe_tag::package::Package;
use cpe_tag::query_builder::{get_grep_patterns, query};
use os_adapter::adapter::{get_adapter, OsAdapter};
use reqwest::Client;
use security_advisories::cve_summary::CveSummary;
use security_advisories::http::get_client;
use security_advisories::service::{
    fetch_cves_by_cpe, fetch_known_exploited_cves, get_cves_summary, CPE_MATCH_FEED,
};
use std::error::Error;
use std::fs::create_dir_all;
use std::fs::read_dir;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

pub async fn execute(
    feed_dir: PathBuf,
    out_dir: PathBuf,
    pkg_dir: Option<PathBuf>,
    recursive: bool,
) -> Result<(), Box<dyn Error>> {
    // todo: progress bar
    let now = Utc::now();
    let [date, time] = [
        now.date().to_string(),
        format!("{:02}:{:02}:{:02}Z", now.hour(), now.minute(), now.second()),
    ];
    let out_dir = out_dir.join(date).join(time);
    let feed = feed_dir.join(CPE_MATCH_FEED);
    let client = get_client()?;

    log::info!("working in {:?} ...", out_dir);
    create_dir_all(&out_dir)?;
    let known_exploited_cves = fetch_known_exploited_cves(&client).await?;

    log::debug!("getting os adapter ...");
    if !recursive {
        let os = get_adapter(pkg_dir)?;
        scan(&*os, &out_dir, &client, &feed, &known_exploited_cves).await?;
    } else {
        let mut os = get_adapter(None)?;
        let kits_dir = &pkg_dir.unwrap().join("kits");
        for kit in read_dir(&kits_dir)? {
            os.set_pkg_dir(kit?.path());
            scan(&*os, &out_dir, &client, &feed, &known_exploited_cves).await?;
        }
    }

    println!("Done. You can find results in {:?}", out_dir.as_os_str());
    Ok(())
}

async fn scan(
    os: &'_ dyn OsAdapter,
    out_dir: &Path,
    client: &Client,
    feed: &Path,
    known_exploited_cves: &[String],
) -> Result<(), Box<dyn Error>> {
    log::info!("listing all catpkgs ...");
    let catpkgs = os.get_all_catpkgs()?;

    for (ctg, pkgs) in catpkgs {
        if pkgs.is_empty() {
            continue;
        }

        let cwd = out_dir.join(&ctg);
        log::debug!("processing {} ...", ctg);
        handle_pkgs(client, feed, &cwd, &ctg, &pkgs, known_exploited_cves).await?;
    }
    Ok(())
}

async fn handle_pkgs(
    client: &Client,
    feed: &Path,
    cwd: &Path,
    category: &str,
    pkgs: &[Package],
    known_exploited_cves: &[String],
) -> Result<(), Box<dyn Error>> {
    let pattern = get_grep_patterns(pkgs)?;
    let matches = query(pattern, feed)?;

    if matches.is_empty() {
        log::info!(
            "no CPE matches in {}. This *MIGHT* indicate false negatives ...",
            category
        );
        return Ok(());
    }

    log::info!(
        "found CPEs for packages in {}. Searching for CVEs ...",
        category
    );
    handle_cves(client, cwd, category, &matches, known_exploited_cves).await
}

async fn handle_cves(
    client: &Client,
    cwd: &Path,
    category: &str,
    matches: &[String],
    known_exploited_cves: &[String],
) -> Result<(), Box<dyn Error>> {
    let mut already_notified = false;
    for cpe in matches {
        let cves = match fetch_cves_by_cpe(client, cpe).await {
            Ok(res) => get_cves_summary(&res, Some(known_exploited_cves)),
            Err(e) => {
                log::error!("{}", e);
                vec![]
            }
        };

        if cves.is_empty() {
            continue;
        }

        if !already_notified {
            log::warn!("found CVEs in {} ...", category);
            already_notified = true;
        }
        write_report(cwd, cpe, &cves)?;
    }

    Ok(())
}

fn write_report(cwd: &Path, cpe: &str, cves: &[CveSummary]) -> Result<(), Box<dyn Error>> {
    log::info!("saving report in {:?} ...", cwd.as_os_str());
    create_dir_all(cwd)?;
    let mut f = File::create(cwd.join(format!("{}.txt", cpe)))?;

    for cve in cves {
        log::debug!("{}", cve.id);
        writeln!(f, "{}", cve)?;
    }
    Ok(())
}
