/*
 * SPDX-License-Identifier: MPL-2.0
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
use crate::conf::ApiKeys;
use chrono::{Timelike, Utc};
use cpe_tag::package::Package;
use cpe_tag::query_builder::{get_regex_pattern, get_value};
use os_adapter::adapter::{get_adapter, OsAdapter};
use rayon::prelude::*;
use regex::Regex;
use reqwest::Client;
use security_advisories::cve_summary::CveSummary;
use security_advisories::http::get_client;
use security_advisories::service::{
    fetch_cves_by_cpe, fetch_known_exploited_cves, get_cves_summary, CPE_MATCH_FEED,
};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::{create_dir_all, read_dir, set_permissions, File};
use std::io::{self, BufRead, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

pub async fn execute(
    feed_dir: PathBuf,
    out_dir: PathBuf,
    pkg_dir: Option<PathBuf>,
    recursive: bool,
    api_keys: ApiKeys,
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
            &feed,
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
                &feed,
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
    feed: &Path,
    known_exploited_cves: &[String],
    api_keys: &ApiKeys,
) -> Result<(), Box<dyn Error>> {
    log::info!("listing all catpkgs ...");
    let catpkgs = os.get_all_catpkgs()?;
    let mut feed_buffer = HashSet::new();
    load_feed(feed, &mut feed_buffer)?;

    for (ctg, pkgs) in catpkgs {
        if pkgs.is_empty() {
            continue;
        }

        log::debug!("processing {} ...", ctg);
        let cwd = out_dir.join(&ctg);
        let mut regex_buffer = vec![];
        let mut result_buffer = vec![];

        load_regex(pkgs, &mut regex_buffer)?;
        regex_buffer
            .par_iter()
            .map(|(pkg, re)| match_cpes(&feed_buffer, pkg, re))
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

fn load_feed(feed: &Path, buffer: &mut HashSet<String>) -> Result<(), Box<dyn Error>> {
    log::debug!("loading feed into memory ...");
    let file = File::open(feed)?;
    let lines = io::BufReader::new(file).lines();
    for line in lines.flatten() {
        if line.contains("cpe23Uri") {
            buffer.insert(get_value(&line));
        }
    }
    Ok(())
}

fn load_regex(
    pkgs: Vec<Package>,
    buffer: &mut Vec<(Package, Regex)>,
) -> Result<(), Box<dyn Error>> {
    for pkg in pkgs {
        let pattern = get_regex_pattern(&[pkg.clone()])?;
        let re = Regex::new(pattern.as_ref())?;
        buffer.push((pkg, re));
    }
    Ok(())
}

fn match_cpes<'a>(
    feed: &'a HashSet<String>,
    pkg: &'a Package,
    re: &'a Regex,
) -> HashMap<&'a Package, Vec<String>> {
    let mut cpes = HashMap::new();
    let matches = feed
        .iter()
        .filter(|feed_entry| re.is_match(feed_entry))
        .map(|x| x.to_owned())
        .collect();
    cpes.insert(pkg, matches);
    cpes
}

async fn handle_pkgs(
    client: &Client,
    cwd: &Path,
    category: &str,
    pkgs: &[HashMap<&Package, Vec<String>>],
    known_exploited_cves: &[String],
    api_keys: &ApiKeys,
) -> Result<(), Box<dyn Error>> {
    for items in pkgs {
        for (pkg, matches) in items {
            let pkg_name = pkg.to_string();

            if matches.is_empty() {
                continue;
            }

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
    Ok(())
}

async fn handle_cves(
    client: &Client,
    cwd: &Path,
    category: &str,
    pkg_name: &str,
    matches: &[String],
    known_exploited_cves: &[String],
    api_keys: &ApiKeys,
) -> Result<(), Box<dyn Error>> {
    let mut already_notified = false;
    for cpe in matches {
        let cves = match fetch_cves_by_cpe(client, cpe, api_keys).await {
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
            log::warn!("found CVEs for {}/{} ...", category, pkg_name);
            already_notified = true;
        }
        write_report(cwd, pkg_name, cves, cpe)?;
    }

    Ok(())
}

fn write_report(
    cwd: &Path,
    pkg_name: &str,
    cves: Vec<CveSummary>,
    cpe: &str,
) -> Result<(), Box<dyn Error>> {
    create_dir_all(cwd)?;
    let f = cwd.join(format!("{}.txt", pkg_name));
    log::debug!("saving report in {:?} ...", f.as_os_str());
    let mut f = File::create(f)?;

    for mut cve in cves {
        cve.related_cpe = Some(cpe.to_owned());
        log::debug!("{}", cve.id);
        writeln!(f, "{}", cve)?;
    }
    Ok(())
}
