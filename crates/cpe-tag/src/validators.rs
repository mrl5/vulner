/*
 * SPDX-License-Identifier: MPL-2.0
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use jsonschema::{Draft, JSONSchema};
use serde_json::from_str;
use std::error::Error;
use std::io;

pub fn validate_batch(batch: &str) -> Result<(), Box<dyn Error>> {
    let instance = from_str(batch)?;
    let compiled = get_batch_schema();

    let res = match compiled.validate(&instance) {
        Ok(_) => Ok(()),
        Err(errors) => {
            for error in errors {
                log::error!("Validation error: {}", error);
                log::error!("Instance path: {}", error.instance_path);
            }

            let err_kind = io::ErrorKind::InvalidInput;
            let err_msg = "Validation error";
            return Err(Box::new(io::Error::new(err_kind, err_msg)));
        }
    };
    res
}

fn get_batch_schema() -> JSONSchema {
    let package_schema = from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/schemas/package.schema.json"
    )))
    .unwrap();
    let batch_schema: serde_json::Value = from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/schemas/batch.schema.json"
    )))
    .unwrap();

    JSONSchema::options()
        .with_draft(Draft::Draft7)
        .with_document(
            "http://localhost/schemas/package".to_owned(),
            package_schema,
        )
        .with_meta_schemas()
        .compile(&batch_schema)
        .expect("valid schema")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_should_validate() {
        let valid_batch = r#"[{"name": "busybox", "versions": [{"version": "1.29.3"}]}]"#;
        let invalid_batch = r#"{"name": "busybox", "versions": [{"version": "1.29.3"}]}"#;
        assert!(validate_batch(valid_batch).is_ok());
        assert!(validate_batch(invalid_batch).is_err());
    }
}
