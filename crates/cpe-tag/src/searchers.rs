/*
 * SPDX-License-Identifier: MPL-2.0
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use grep_matcher::Matcher;
use grep_regex::RegexMatcher;
use grep_searcher::sinks::UTF8;
use grep_searcher::Searcher;
use package_model::package::Package;
use regex::Regex;
use security_advisories::service::CPE_KEYWORD_IN_FEED_LINE;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::path::Path;

pub fn grep(pattern: String, feed: &Path) -> Result<HashSet<String>, Box<dyn Error>> {
    if pattern.is_empty() {
        return Ok(HashSet::new());
    }

    let matcher = RegexMatcher::new_line_matcher(&pattern)?;
    let mut matches: Vec<String> = vec![];

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

    Ok(get_uniq_values(matches))
}

pub fn match_cpes<'a>(
    feed: &[Box<str>],
    pkg: &'a Package,
    re_pattern: &str,
) -> HashMap<&'a Package, HashSet<String>> {
    let mut cpes = HashMap::new();
    cpes.insert(pkg, get_matches(re_pattern, feed));
    cpes
}

pub fn scrap_cpe(line: &str) -> String {
    let s: Vec<&str> = line.rsplit(CPE_KEYWORD_IN_FEED_LINE).collect();
    s[0].trim().trim_matches(',').trim_matches('"').to_owned()
}

pub fn contains_cpe_json_key(line: &str) -> bool {
    line.contains(CPE_KEYWORD_IN_FEED_LINE)
}

fn get_matches(re_pattern: &str, feed: &[Box<str>]) -> HashSet<String> {
    if re_pattern.is_empty() {
        return HashSet::new();
    }

    let re = Regex::new(re_pattern).unwrap();
    feed.iter()
        .filter(|feed_entry| re.is_match(feed_entry))
        .map(|x| x.to_string())
        .collect()
}

fn get_uniq_values(matches: Vec<String>) -> HashSet<String> {
    let values: HashSet<String> = matches.iter().map(|m| scrap_cpe(m)).collect();
    values
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_should_scrap_value() {
        let line = "    \"cpe23Uri\" : \"cpe:2.3:a:xmlsoft:libxml2:2.9.10:*:*:*:*:*:*:*\",\r\n";
        assert_eq!(
            scrap_cpe(line),
            "cpe:2.3:a:xmlsoft:libxml2:2.9.10:*:*:*:*:*:*:*"
        );
    }
    #[test]
    fn it_should_ignore_empty_string_as_a_pattern() {
        let test_matches = [
            Box::from(
                "      \"cpe23Uri\" : \"cpe:2.3:a:busybox:busybox:1.29.3:*:*:*:*:*:*:*\"\r\n",
            ),
            Box::from(
                "      \"cpe23Uri\" : \"cpe:2.3:a:xmlsoft:libxml2:2.9.10:*:*:*:*:*:*:*\"\r\n",
            ),
        ];

        assert_eq!(get_matches("", &test_matches).len(), 0);
    }

    #[test]
    fn it_should_return_unique_values() {
        let test_matches = vec![
            String::from(
                "      \"cpe23Uri\" : \"cpe:2.3:a:busybox:busybox:1.29.3:*:*:*:*:*:*:*\"\r\n",
            ),
            String::from(
                "      \"cpe23Uri\" : \"cpe:2.3:a:busybox:busybox:1.29.3:*:*:*:*:*:*:*\"\r\n",
            ),
            String::from(
                "      \"cpe23Uri\" : \"cpe:2.3:a:xmlsoft:libxml2:2.9.10:*:*:*:*:*:*:*\"\r\n",
            ),
            String::from(
                "      \"cpe23Uri\" : \"cpe:2.3:a:xmlsoft:libxml2:2.9.10:-:*:*:*:*:*:*\"\r\n",
            ),
            String::from(
                "      \"cpe23Uri\" : \"cpe:2.3:a:xmlsoft:libxml2:2.9.10:*:*:*:*:*:*:*\"\r\n",
            ),
            String::from(
                "      \"cpe23Uri\" : \"cpe:2.3:a:xmlsoft:libxml2:2.9.10:-:*:*:*:*:*:*\"\r\n",
            ),
            String::from(
                "    \"cpe23Uri\" : \"cpe:2.3:a:xmlsoft:libxml2:2.9.10:*:*:*:*:*:*:*\",\r\n",
            ),
        ];
        let expected: HashSet<String> = HashSet::from([
            "cpe:2.3:a:busybox:busybox:1.29.3:*:*:*:*:*:*:*".to_owned(),
            "cpe:2.3:a:xmlsoft:libxml2:2.9.10:*:*:*:*:*:*:*".to_owned(),
            "cpe:2.3:a:xmlsoft:libxml2:2.9.10:-:*:*:*:*:*:*".to_owned(),
        ]);

        assert_eq!(get_uniq_values(test_matches), expected);
    }

    #[test]
    fn it_should_recognize_line_containing_cpe_value() {
        let input = vec![
            (false, "{"),
            (false, "  \"matches\" : [ {"),
            (true, "    \"cpe23Uri\" : \"cpe:2.3:a:\\$0.99_kindle_books_project:\\$0.99_kindle_books:6:*:*:*:*:android:*:*\","),
            (false, "    \"cpe_name\" : [ {"),
            (true, "      \"cpe23Uri\" : \"cpe:2.3:a:\\$0.99_kindle_books_project:\\$0.99_kindle_books:6:*:*:*:*:android:*:*\""),
            (false, "    } ]"),
            (false, "  }, {"),
            (true, "    \"cpe23Uri\" : \"cpe:2.3:o:-:-:-:*:*:*:*:*:*:*\","),
            (false, "    \"cpe_name\" : [ ]"),
            (false, "  }, {"),
        ];
        for (expected, line) in input {
            assert_eq!(contains_cpe_json_key(line), expected);
        }
    }
}
