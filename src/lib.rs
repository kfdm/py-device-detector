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
    fn parse(&self, ua: &str, headers: Option<Vec<(String, String)>>) -> PyResult<String> {
        let result = match self.dd.parse(ua, headers)? {
            Detection::Bot(bot) => std::format!("{:?}", bot).to_string(),
            Detection::Known(device) => std::format!("{:?}", device).to_string(),
        };
        Ok(result)
    }
}

/// Formats the sum of two numbers as string.
#[pyfunction]
#[pyo3(signature = (ua, headers=None))]
fn parse(_py: Python, ua: &str, headers: Option<Vec<(String, String)>>) -> PyResult<String> {
    PyDeviceDetector::new(0).parse(ua, headers)
}

/// A Python module implemented in Rust.
#[pymodule]
fn py_device_detector(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    m.add_class::<PyDeviceDetector>()?;
    Ok(())
}
