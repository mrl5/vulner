/*
 * SPDX-License-Identifier: MPL-2.0
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use atty::Stream;
use once_cell::sync::Lazy;
use std::io::{stdin, BufRead, Error, ErrorKind, Stdin};

static STDIN: Lazy<Stdin> = Lazy::new(stdin);

pub fn get_input(input: Option<String>) -> Result<String, Error> {
    match input {
        Some(v) => Ok(v),
        None => {
            if atty::is(Stream::Stdin) {
                return Err(Error::new(ErrorKind::Other, "stdin not redirected"));
            }
            handle_stdin()
        }
    }
}

fn handle_stdin() -> Result<String, Error> {
    let mut buffer = String::new();
    let mut v = STDIN.lock();
    v.read_line(&mut buffer)?;

    Ok(buffer)
}
