use super::CpeQueryBuilder;
use pyo3::prelude::*;
use std::error::Error;

pub struct PythonAdapter {}

impl CpeQueryBuilder for PythonAdapter {
    fn get_grep_patterns(&self, serialized_json: &str) -> Result<String, Box<dyn Error>> {
        let py_errors = include_str!("../../python/cpe_tag/errors.py");
        let py_cpe = include_str!("../../python/cpe_tag/cpe.py");
        let py_serializers = include_str!("../../python/cpe_tag/serializers.py");
        let py_integrator = include_str!("../../python/integrator.py");

        let from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
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
                ":busybox:1\\.31\\.0:[\\*\\-]:[^:]+:[^:]+:[^:]+:(linux|\\*):[^:]+:[^:]"
            ],
            [
                r#"{"name":"busybox","versions":[{"version":"1.31.0"},{"version":"1.29.3"}]}"#,
                ":busybox:1\\.31\\.0:[\\*\\-]:[^:]+:[^:]+:[^:]+:(linux|\\*):[^:]+:[^:]|:busybox:1\\.29\\.3:[\\*\\-]:[^:]+:[^:]+:[^:]+:(linux|\\*):[^:]+:[^:]"
            ],
            [
                r#"[{"name":"libxml2","versions":[{"version":"2.9.10-r5"}]},{"name":"openssh","versions":[{"version":"8.4_p1-r3"}]}]"#,
                ":libxml2:2\\.9\\.10:[\\*\\-]:[^:]+:[^:]+:[^:]+:(linux|\\*):[^:]+:[^:]|:openssh:8\\.4:(p1|\\*):[^:]+:[^:]+:[^:]+:(linux|\\*):[^:]+:[^:]"
            ],
            [
                r#"{"name":"google-chrome","versions":[{"version":"97.0.4692.71"}]}"#,
                "google:chrome:97\\.0\\.4692\\.71:[\\*\\-]:[^:]+:[^:]+:[^:]+:(linux|\\*):[^:]+:[^:]"
            ],
            [
                r#"{"name":"nicotine+","versions":[{"version":"1.4.1-r1"}]}"#,
                ":nicotine\\+:1\\.4\\.1:[\\*\\-]:[^:]+:[^:]+:[^:]+:(linux|\\*):[^:]+:[^:]"
            ]
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
