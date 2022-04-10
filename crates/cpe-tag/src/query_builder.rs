/*
 * SPDX-License-Identifier: MPL-2.0
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::package::Package;
use std::error::Error;
mod py_query_builder;

pub fn get_regex_pattern(packages: &[Package]) -> Result<String, Box<dyn Error>> {
    py_query_builder::get_regex_pattern(packages)
}
