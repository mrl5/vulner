/*
 * SPDX-License-Identifier: MPL-2.0
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
use crate::conf::ApiKeys;
use crate::utils::{get_feed_path, get_memory_size};
use cpe_tag::package::Package;
use cpe_tag::query_builder::get_regex_pattern;
use cpe_tag::searchers::{contains_cpe_json_key, match_cpes, scrap_cpe};
use os_adapter::adapter::{get_adapter, OsAdapter};
use rayon::prelude::*;
use reqwest::Client;
use security_advisories::cve_summary::CveSummary;
use security_advisories::http::get_client;
use security_advisories::service::{
    fetch_cves_by_cpe, fetch_known_exploited_cves, get_cves_summary,
};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::{create_dir_all, read_dir, set_permissions, File, OpenOptions};
use std::io::{self, BufRead, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use time::OffsetDateTime;

pub async fn execute(
    feed_dir: PathBuf,
    out_dir: PathBuf,
    pkg_dir: Option<PathBuf>,
    recursive: bool,
    api_keys: ApiKeys,
) -> Result<(), Box<dyn Error>> {
    // todo: progress bar
    let now = OffsetDateTime::now_utc();
    let [date, time] = [
        now.date().to_string(),
        format!("{:02}:{:02}:{:02}Z", now.hour(), now.minute(), now.second()),
    ];
    let out_dir = out_dir.join(date).join(time);
    let client = get_client()?;

    log::info!("working in {:?} ...", out_dir);
    create_dir_all(&out_dir)?;
    let metadata = out_dir.metadata()?;
    let mut permissions = metadata.permissions();
    permissions.set_mode(0o700);
    set_permissions(&out_dir, permissions)?;

    let known_exploited_cves = fetch_known_exploited_cves(&client).await?;

    log::debug!("getting os adapter ...");
    if !recursive {
        let os = get_adapter(pkg_dir)?;
        scan(
            &*os,
            &out_dir,
            &client,
            &feed_dir,
            &known_exploited_cves,
            &api_keys,
        )
        .await?;
    } else {
        let mut os = get_adapter(None)?;
        let kits_dir = &pkg_dir.unwrap().join("kits");
        for kit in read_dir(&kits_dir)? {
            os.set_pkg_dir(kit?.path());
            scan(
                &*os,
                &out_dir,
                &client,
                &feed_dir,
                &known_exploited_cves,
                &api_keys,
            )
            .await?;
        }
    }

    println!("Done. You can find results in {:?}", out_dir.as_os_str());
    Ok(())
}

async fn scan(
    os: &'_ dyn OsAdapter,
    out_dir: &Path,
    client: &Client,
    feed_dir: &Path,
    known_exploited_cves: &[String],
    api_keys: &ApiKeys,
) -> Result<(), Box<dyn Error>> {
    log::info!("listing all catpkgs ...");
    let catpkgs = os.get_all_catpkgs()?;
    let feed_buffer = load_feed(feed_dir)?;
    log::info!(
        "allocated {} bytes in memory for feed buffer",
        get_memory_size(&feed_buffer)
    );

    for (ctg, pkgs) in catpkgs {
        if pkgs.is_empty() {
            continue;
        }

        log::debug!("processing {} ...", ctg);
        let cwd = out_dir.join(&ctg);
        let mut pattern_buffer = vec![];
        let mut result_buffer = vec![];

        load_patterns(pkgs, &mut pattern_buffer)?;
        pattern_buffer
            .par_iter()
            .map(|(pkg, re_pattern)| match_cpes(&feed_buffer, pkg, re_pattern))
            .collect_into_vec(&mut result_buffer);
        handle_pkgs(
            client,
            &cwd,
            &ctg,
            &result_buffer,
            known_exploited_cves,
            api_keys,
        )
        .await?;
    }
    Ok(())
}

fn load_feed(feed_dir: &Path) -> Result<Vec<Box<str>>, Box<dyn Error>> {
    log::info!("loading feed into memory ...");
    let mut buffer = HashSet::new();
    let feed = get_feed_path(feed_dir);
    let file = File::open(feed)?;
    let lines = io::BufReader::new(file).lines();
    for line in lines.flatten() {
        if contains_cpe_json_key(&line) {
            buffer.insert(scrap_cpe(&line).into_boxed_str());
        }
    }
    Ok(buffer.into_iter().collect())
}

fn load_patterns(
    pkgs: Vec<Package>,
    buffer: &mut Vec<(Package, Box<str>)>,
) -> Result<(), Box<dyn Error>> {
    for pkg in pkgs {
        let pattern = get_regex_pattern(&[pkg.clone()])?;
        buffer.push((pkg, pattern.into_boxed_str()));
    }
    Ok(())
}

async fn handle_pkgs(
    client: &Client,
    cwd: &Path,
    category: &str,
    pkgs: &[HashMap<&Package, HashSet<String>>],
    known_exploited_cves: &[String],
    api_keys: &ApiKeys,
) -> Result<(), Box<dyn Error>> {
    let mut any_cpes = false;
    for items in pkgs {
        for (pkg, matches) in items {
            let pkg_name = pkg.to_string();

            if matches.is_empty() {
                continue;
            }

            any_cpes = true;
            log::debug!(
                "found CPE(s) for {}/{}. Searching for CVEs ...",
                category,
                &pkg_name
            );
            handle_cves(
                client,
                cwd,
                category,
                &pkg_name,
                matches,
                known_exploited_cves,
                api_keys,
            )
            .await?;
        }
    }

    if !any_cpes {
        log::info!(
            "no CPEs in {} - this might indicate false negatives ...",
            category
        );
    }
    Ok(())
}

async fn handle_cves(
    client: &Client,
    cwd: &Path,
    category: &str,
    pkg_name: &str,
    matches: &HashSet<String>,
    known_exploited_cves: &[String],
    api_keys: &ApiKeys,
) -> Result<(), Box<dyn Error>> {
    let mut already_notified = false;
    let mut cves: HashSet<CveSummary> = HashSet::new();

    for cpe in matches {
        match fetch_cves_by_cpe(client, cpe, api_keys).await {
            Ok(res) => {
                for cve in get_cves_summary(&res, Some(known_exploited_cves)) {
                    cves.insert(cve);
                }
            }
            Err(e) => {
                log::error!("{category}/{pkg_name}: {e}");
            }
        };

        if cves.is_empty() {
            continue;
        }

        if !already_notified {
            log::warn!("found CVEs for {}/{} ...", category, pkg_name);
            already_notified = true;
        }
    }

    if !cves.is_empty() {
        write_report(cwd, pkg_name, &cves)
    } else {
        Ok(())
    }
}

fn write_report(
    cwd: &Path,
    pkg_name: &str,
    cves: &HashSet<CveSummary>,
) -> Result<(), Box<dyn Error>> {
    create_dir_all(cwd)?;
    let f = cwd.join(format!("{}.txt", pkg_name));
    log::debug!("saving report in {:?} ...", f.as_os_str());

    let mut buffer = OpenOptions::new().create(true).append(true).open(f)?;
    for cve in cves {
        log::debug!("{}", cve.id);
        writeln!(buffer, "{}", cve)?;
    }

    Ok(())
}
