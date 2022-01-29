/*
 * SPDX-License-Identifier: MPL-2.0
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::validators::Package;
use pyo3::prelude::*;
use pythonize::pythonize;
use std::error::Error;

pub fn get_grep_patterns(packages: &[Package]) -> Result<String, Box<dyn Error>> {
    let from_python = call_python_code(packages);
    Ok(from_python?.to_string())
}

fn call_python_code(payload: &[Package]) -> PyResult<Py<PyAny>> {
    let modules = get_py_modules();
    let py_integrator = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/python/integrator.py"));

    log::debug!("acquiring the global interpreter lock");
    Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        log::debug!("importing python files as modules ...");
        for module in modules {
            let [code, name] = module;
            PyModule::from_code(py, code, name, name)?;
        }
        let integrator: Py<PyAny> = PyModule::from_code(py, py_integrator, "", "")?
            .getattr("run")?
            .into();

        let packages = pythonize(py, payload)?;
        log::debug!("executing python code ...");
        integrator.call1(py, (packages,))
    })
}

fn get_py_modules() -> Vec<[&'static str; 2]> {
    let py_conf = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/python/cpe_tag/conf.py"
    ));
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

    vec![
        [py_conf, "cpe_tag.conf"],
        [py_errors, "cpe_tag.errors"],
        [py_cpe, "cpe_tag.cpe"],
        [py_serializers, "cpe_tag.serializers"],
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validators::into_validated_packages;
    use serde_json::{from_str, Value};

    #[test]
    fn it_should_return_grep_patterns() {
        let test_data = [
            [
                r#"[{"name":"busybox","versions":[{"version":"1.31.0"},{"version":"9999"}]}]"#,
                r#":busybox:1\.31\.0:[\*\-]:[^:]+:[^:]+:[^:]+:(linux|\*):[^:]+:[^:]"#,
            ],
            [
                r#"[{"name":"busybox","versions":[{"version":"1.31.0"},{"version":"1.29.3"}]}]"#,
                r#":busybox:1\.31\.0:[\*\-]:[^:]+:[^:]+:[^:]+:(linux|\*):[^:]+:[^:]|:busybox:1\.29\.3:[\*\-]:[^:]+:[^:]+:[^:]+:(linux|\*):[^:]+:[^:]"#,
            ],
            [
                r#"[{"name":"libxml2","versions":[{"version":"2.9.10-r5"}]},{"name":"openssh","versions":[{"version":"8.4_p1-r3"}]}]"#,
                r#":libxml2:2\.9\.10:[\*\-]:[^:]+:[^:]+:[^:]+:(linux|\*):[^:]+:[^:]|:openssh:8\.4:(p1|\*):[^:]+:[^:]+:[^:]+:(linux|\*):[^:]+:[^:]"#,
            ],
            [
                r#"[{"name":"google-chrome","versions":[{"version":"97.0.4692.71"}]}]"#,
                r#"google:chrome:97\.0\.4692\.71:[\*\-]:[^:]+:[^:]+:[^:]+:(linux|\*):[^:]+:[^:]"#,
            ],
            [
                r#"[{"name":"nicotine+","versions":[{"version":"1.4.1-r1"}]}]"#,
                r#":nicotine\+:1\.4\.1:[\*\-]:[^:]+:[^:]+:[^:]+:(linux|\*):[^:]+:[^:]"#,
            ],
        ];
        for d in test_data {
            let [json, expected] = d;
            let json: Value = from_str(json).unwrap();
            let json = into_validated_packages(&json).unwrap();
            assert_eq!(get_grep_patterns(&json).unwrap(), expected.to_owned());
        }
    }
}
