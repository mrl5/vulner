/*
 * SPDX-License-Identifier: MPL-2.0
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use jsonschema::{Draft, JSONSchema};
use serde_json::{from_str, Value};
use std::error::Error;
use std::io;

pub fn validate_packages_batch(batch: &Value) -> Result<(), Box<dyn Error>> {
    validate(batch, get_packages_batch_schema())
}

pub fn validate_cpe_batch(batch: &Value) -> Result<(), Box<dyn Error>> {
    validate(batch, get_cpe_batch_schema())
}

fn validate(instance: &Value, compiled: JSONSchema) -> Result<(), Box<dyn Error>> {
    let res = match compiled.validate(instance) {
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

fn get_packages_batch_schema() -> JSONSchema {
    let package_schema = from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/schemas/package.schema.json"
    )))
    .unwrap();
    let batch_schema: serde_json::Value = from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/schemas/packages-batch.schema.json"
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

fn get_cpe_batch_schema() -> JSONSchema {
    let package_schema = from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/schemas/cpe.schema.json"
    )))
    .unwrap();
    let batch_schema: serde_json::Value = from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/schemas/cpe-batch.schema.json"
    )))
    .unwrap();

    JSONSchema::options()
        .with_draft(Draft::Draft7)
        .with_document("http://localhost/schemas/cpe".to_owned(), package_schema)
        .with_meta_schemas()
        .compile(&batch_schema)
        .expect("valid schema")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_validate_packages_batch() {
        let valid_batch =
            from_str(r#"[{"name": "busybox", "versions": [{"version": "1.29.3"}]}]"#).unwrap();
        let invalid_batch =
            from_str(r#"{"name": "busybox", "versions": [{"version": "1.29.3"}]}"#).unwrap();
        assert!(validate_packages_batch(&valid_batch).is_ok());
        assert!(validate_packages_batch(&invalid_batch).is_err());
    }

    #[test]
    fn it_should_validate_cpe_batch() {
        let valid_batch =
            from_str(r#"["cpe:2.3:a:busybox:busybox:1.29.3:*:*:*:*:*:*:*"]"#).unwrap();
        let invalid_batch = from_str(r#"["cpe:/a:busybox:busybox:1.29.3 "]"#).unwrap();
        assert!(validate_cpe_batch(&valid_batch).is_ok());
        assert!(validate_cpe_batch(&invalid_batch).is_err());
    }
}
