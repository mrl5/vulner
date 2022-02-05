/*
 * SPDX-License-Identifier: MPL-2.0
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::package::Package;
use grep::matcher::Matcher;
use grep::regex::RegexMatcher;
use grep::searcher::sinks::UTF8;
use grep::searcher::Searcher;
use std::error::Error;
use std::path::Path;
mod py_query_builder;

pub fn get_grep_patterns(packages: &[Package]) -> Result<String, Box<dyn Error>> {
    py_query_builder::get_grep_patterns(packages)
}

pub fn query(pattern: String, feed: &Path) -> Result<Vec<String>, Box<dyn Error>> {
    let matcher = RegexMatcher::new_line_matcher(&pattern)?;
    let mut matches: Vec<String> = vec![];
    if !feed.exists() {
        log::error!("{:?} doesn't exist", feed.as_os_str());
    }

    Searcher::new().search_path(
        &matcher,
        feed,
        UTF8(|_, line| match matcher.find(line.as_bytes())? {
            Some(_) => {
                matches.push(line.to_owned());
                Ok(true)
            }
            None => Ok(false),
        }),
    )?;

    Ok(get_values(matches))
}

fn get_values(matches: Vec<String>) -> Vec<String> {
    let mut values: Vec<String> = vec![];
    for m in matches {
        let s: Vec<&str> = m.rsplit(" : ").collect();
        values.push(s[0].trim().trim_matches(',').trim_matches('"').to_owned());
    }
    values.sort();
    values.dedup();
    values
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_should_return_unique_values() {
        let test_matches = vec![
            "      \"cpe23Uri\" : \"cpe:2.3:a:busybox:busybox:1.29.3:*:*:*:*:*:*:*\"\r\n"
                .to_owned(),
            "      \"cpe23Uri\" : \"cpe:2.3:a:busybox:busybox:1.29.3:*:*:*:*:*:*:*\"\r\n"
                .to_owned(),
            "      \"cpe23Uri\" : \"cpe:2.3:a:xmlsoft:libxml2:2.9.10:*:*:*:*:*:*:*\"\r\n"
                .to_owned(),
            "      \"cpe23Uri\" : \"cpe:2.3:a:xmlsoft:libxml2:2.9.10:-:*:*:*:*:*:*\"\r\n"
                .to_owned(),
            "      \"cpe23Uri\" : \"cpe:2.3:a:xmlsoft:libxml2:2.9.10:*:*:*:*:*:*:*\"\r\n"
                .to_owned(),
            "      \"cpe23Uri\" : \"cpe:2.3:a:xmlsoft:libxml2:2.9.10:-:*:*:*:*:*:*\"\r\n"
                .to_owned(),
            "    \"cpe23Uri\" : \"cpe:2.3:a:xmlsoft:libxml2:2.9.10:*:*:*:*:*:*:*\",\r\n".to_owned(),
        ];
        let expected = vec![
            "cpe:2.3:a:busybox:busybox:1.29.3:*:*:*:*:*:*:*".to_owned(),
            "cpe:2.3:a:xmlsoft:libxml2:2.9.10:*:*:*:*:*:*:*".to_owned(),
            "cpe:2.3:a:xmlsoft:libxml2:2.9.10:-:*:*:*:*:*:*".to_owned(),
        ];

        assert_eq!(get_values(test_matches), expected);
    }
}
