use std::collections::HashMap;

use pyo3::prelude::*;
use rust_device_detector::device_detector::{Detection, DeviceDetector};

#[pyclass(subclass, name = "DeviceDetector", module = "py_device_detector")]
#[derive(Clone)]
pub struct PyDeviceDetector {
    dd: DeviceDetector,
}

impl PyDeviceDetector {
    pub fn create(py: Python, entries: u64) -> PyResult<PyObject> {
        let pdd = PyDeviceDetector {
            dd: DeviceDetector::new_with_cache(entries),
        };
        Ok(Py::new(py, pdd)?.into_py(py))
    }
}

#[pymethods]
impl PyDeviceDetector {
    #[new]
    pub fn new(entries: u64) -> Self {
        PyDeviceDetector {
            dd: DeviceDetector::new_with_cache(entries),
        }
    }

    #[pyo3(signature = (ua, headers=None))]
    fn parse(
        &self,
        ua: &str,
        headers: Option<Vec<(String, String)>>,
    ) -> PyResult<HashMap<String, String>> {
        let result = match self.dd.parse(ua, headers)? {
            Detection::Bot(bot) => PyBot(bot).to_hashmap(),
            Detection::Known(device) => PyDevice(device).to_hashmap(),
        };
        Ok(result)
    }
}

#[pyclass(subclass, name = "Bot", module = "py_device_detector")]
#[derive(Clone, Debug)]
pub struct PyBot(rust_device_detector::device_detector::Bot);

impl PyBot {
    fn to_hashmap(&self) -> HashMap<String, String> {
        let mut data = HashMap::new();
        data.insert("name".to_string(), self.0.name.clone());
        if let Some(category) = self.0.category.clone() {
            data.insert("category".to_string(), category);
        }
        if let Some(url) = self.0.url.clone() {
            data.insert("url".to_string(), url);
        }

        data
    }
}

#[pyclass(subclass, name = "KnownDevice", module = "py_device_detector")]
#[derive(Clone, Debug)]
pub struct PyDevice(rust_device_detector::device_detector::KnownDevice);

impl PyDevice {
    fn to_hashmap(&self) -> HashMap<String, String> {
        let mut data = HashMap::new();
        if let Some(client) = self.0.client.clone() {
            data.insert(
                "client".to_string(),
                std::format!("{:?}", client).to_string(),
            );
        }
        if let Some(device) = self.0.device.clone() {
            data.insert(
                "device".to_string(),
                std::format!("{:?}", device).to_string(),
            );
        }
        if let Some(os) = self.0.os.clone() {
            let mut inner = HashMap::new();
            if let Some(family) = os.family {
                inner.insert("family".to_string(), family);
            }
            // data.insert("os".to_string(),inner );
        }
        data
    }
}

/// Formats the sum of two numbers as string.
#[pyfunction]
#[pyo3(signature = (ua, headers=None))]
fn parse(
    _py: Python,
    ua: &str,
    headers: Option<Vec<(String, String)>>,
) -> PyResult<HashMap<String, String>> {
    PyDeviceDetector::new(0).parse(ua, headers)
}

/// A Python module implemented in Rust.
#[pymodule]
fn py_device_detector(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    m.add_class::<PyDeviceDetector>()?;
    Ok(())
}
