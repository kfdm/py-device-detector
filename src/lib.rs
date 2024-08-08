use pyo3::prelude::*;
use rust_device_detector::device_detector::{Detection, DeviceDetector};

mod internal;
use self::internal::{BotWrapper, DeviceWrapper};

#[pyclass(subclass, name = "DeviceDetector", module = "py_device_detector")]
#[derive(Clone)]
pub struct PyDeviceDetector(DeviceDetector);

impl PyDeviceDetector {
    pub fn create(py: Python, entries: u64) -> PyResult<PyObject> {
        let pdd = PyDeviceDetector::new(entries);
        Ok(Py::new(py, pdd)?.into_py(py))
    }
}

#[pymethods]
impl PyDeviceDetector {
    #[new]
    pub fn new(entries: u64) -> Self {
        PyDeviceDetector(DeviceDetector::new_with_cache(entries))
    }

    #[pyo3(signature = (ua, headers=None))]
    fn parse(&self, ua: &str, headers: Option<Vec<(String, String)>>) -> PyResult<PyObject> {
        Python::with_gil(|py| -> PyResult<PyObject> {
            match self.0.parse(ua, headers)? {
                Detection::Bot(bot) => BotWrapper(bot).to_object(py),
                Detection::Known(device) => DeviceWrapper(device).to_object(py),
            }
        })
    }
}

/// Parse the useragent
#[pyfunction]
#[pyo3(signature = (ua, headers=None))]
fn parse(_py: Python, ua: &str, headers: Option<Vec<(String, String)>>) -> PyResult<PyObject> {
    PyDeviceDetector::new(0).parse(ua, headers)
}

/// A Python module implemented in Rust.
#[pymodule]
fn py_device_detector(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    m.add_class::<PyDeviceDetector>()?;
    Ok(())
}
