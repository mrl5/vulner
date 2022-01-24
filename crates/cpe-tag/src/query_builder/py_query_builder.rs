/*
 * SPDX-License-Identifier: MPL-2.0
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use super::CpeQueryBuilder;
use pyo3::prelude::*;
use std::error::Error;

pub struct PythonAdapter {}

impl CpeQueryBuilder for PythonAdapter {
    fn get_grep_patterns(&self, serialized_json: &str) -> Result<String, Box<dyn Error>> {
        let py_errors = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/python/cpe_tag/errors.py"
        ));
        let py_cpe = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/python/cpe_tag/cpe.py"
        ));
        let py_serializers = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/python/cpe_tag/serializers.py"
        ));
        let py_integrator =
            include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/python/integrator.py"));

        log::debug!("Acquiring the global interpreter lock");
        let from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
            log::debug!("importing python files as modules ...");
            PyModule::from_code(py, py_errors, "cpe_tag.errors", "cpe_tag.errors")?;
            PyModule::from_code(py, py_cpe, "cpe_tag.cpe", "cpe_tag.cpe")?;
            PyModule::from_code(
                py,
                py_serializers,
                "cpe_tag.serializers",
                "cpe_tag.serializers",
            )?;
            let integrator: Py<PyAny> = PyModule::from_code(py, py_integrator, "", "")?
                .getattr("run")?
                .into();

            log::debug!("executing python code ...");
            integrator.call1(py, (serialized_json,))
        });

        Ok(from_python?.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_should_return_grep_patterns() {
        let py_adapter = PythonAdapter {};
        let test_data = [
            [
                r#"{"name":"busybox","versions":[{"version":"1.31.0"},{"version":"9999"}]}"#,
                r#":busybox:1\.31\.0:[\*\-]:[^:]+:[^:]+:[^:]+:(linux|\*):[^:]+:[^:]"#,
            ],
            [
                r#"{"name":"busybox","versions":[{"version":"1.31.0"},{"version":"1.29.3"}]}"#,
                r#":busybox:1\.31\.0:[\*\-]:[^:]+:[^:]+:[^:]+:(linux|\*):[^:]+:[^:]|:busybox:1\.29\.3:[\*\-]:[^:]+:[^:]+:[^:]+:(linux|\*):[^:]+:[^:]"#,
            ],
            [
                r#"[{"name":"libxml2","versions":[{"version":"2.9.10-r5"}]},{"name":"openssh","versions":[{"version":"8.4_p1-r3"}]}]"#,
                r#":libxml2:2\.9\.10:[\*\-]:[^:]+:[^:]+:[^:]+:(linux|\*):[^:]+:[^:]|:openssh:8\.4:(p1|\*):[^:]+:[^:]+:[^:]+:(linux|\*):[^:]+:[^:]"#,
            ],
            [
                r#"{"name":"google-chrome","versions":[{"version":"97.0.4692.71"}]}"#,
                r#"google:chrome:97\.0\.4692\.71:[\*\-]:[^:]+:[^:]+:[^:]+:(linux|\*):[^:]+:[^:]"#,
            ],
            [
                r#"{"name":"nicotine+","versions":[{"version":"1.4.1-r1"}]}"#,
                r#":nicotine\+:1\.4\.1:[\*\-]:[^:]+:[^:]+:[^:]+:(linux|\*):[^:]+:[^:]"#,
            ],
        ];
        for d in test_data {
            let [serialized_json, expected] = d;
            assert_eq!(
                py_adapter.get_grep_patterns(serialized_json).unwrap(),
                expected.to_owned()
            );
        }
    }
}
