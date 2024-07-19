use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::exceptions::PyOSError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyString};
use rust_device_detector::device_detector::{Detection, DeviceDetector};
use std::fmt;

#[derive(Debug)]
struct MyError {
    pub msg: &'static str,
}

impl std::error::Error for MyError {}
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error from Rust: {}", self.msg)
    }
}
impl std::convert::From<MyError> for PyErr {
    fn from(err: MyError) -> PyErr {
        PyOSError::new_err(err.to_string())
    }
}

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

    fn parse(&self, ua: &str, headers: Option<Vec<(String, String)>>) -> PyResult<String> {
        println!("{:?}", self.dd.parse(ua, None));
        match self.dd.parse(ua, headers) {
            Ok(Detection::Bot(bot)) => Ok("Bot".to_string()),
            Ok(Detection::Known(device)) => Ok("Device".to_string()),
            Err(error) => Err(PyErr::from(MyError {
                msg: "number is less than or equal to 2",
            })),
        }
    }
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn parse(py: Python, ua: &str) -> PyResult<String> {
    let detector = DeviceDetector::new_with_cache(128);
    let result = detector.parse(ua, None);
    Ok("Testing".to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn py_device_detector(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    m.add_class::<PyDeviceDetector>()?;
    Ok(())
}
