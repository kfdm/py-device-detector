use pyo3::{prelude::*, types::PyDict};
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
    fn parse(&self, ua: &str, headers: Option<Vec<(String, String)>>) -> PyResult<PyObject> {
        match self.dd.parse(ua, headers)? {
            Detection::Bot(bot) => PyBot(bot).to_hashmap(),
            Detection::Known(device) => PyDevice(device).to_hashmap(),
        }
    }
}

#[pyclass(subclass, name = "Bot", module = "py_device_detector")]
#[derive(Clone, Debug)]
pub struct PyBot(rust_device_detector::device_detector::Bot);

impl PyBot {
    fn to_hashmap(&self) -> PyResult<PyObject> {
        Python::with_gil(|py| -> PyResult<PyObject> {
            let dict = PyDict::new_bound(py);
            dict.set_item("name", self.0.name.clone());
            if let Some(category) = self.0.category.clone() {
                dict.set_item("category", category);
            }
            if let Some(url) = self.0.url.clone() {
                dict.set_item("url", url);
            }
            // Decode BotProducer
            if let Some(producer) = self.0.producer.clone() {
                let inner = PyDict::new_bound(py);
                if let Some(name) = producer.name {
                    inner.set_item("name", name);
                }
                if let Some(url) = producer.url {
                    inner.set_item("url", url);
                }

                dict.set_item("producer", inner);
            }
            dict.as_any().extract()
        })
    }
}

#[pyclass(subclass, name = "KnownDevice", module = "py_device_detector")]
#[derive(Clone, Debug)]
pub struct PyDevice(rust_device_detector::device_detector::KnownDevice);

impl PyDevice {
    fn to_hashmap(&self) -> PyResult<PyObject> {
        Python::with_gil(|py| -> PyResult<PyObject> {
            let dict = PyDict::new_bound(py);
            // Decode Client
            if let Some(client) = self.0.client.clone() {
                let inner = PyDict::new_bound(py);

                inner.set_item("name", client.name);
                inner.set_item("type", client.r#type.as_str());

                if let Some(version) = client.version {
                    inner.set_item("version", version);
                }
                if let Some(engine) = client.engine {
                    inner.set_item("engine", engine);
                }
                if let Some(engine_version) = client.engine_version {
                    inner.set_item("engine_version", engine_version);
                }

                dict.set_item("client", inner);
            }
            // Decode Device
            if let Some(device) = self.0.device.clone() {
                let inner = PyDict::new_bound(py);
                if let Some(brand) = device.brand {
                    inner.set_item("brand", brand);
                }
                if let Some(model) = device.model {
                    inner.set_item("model", model);
                }
                dict.set_item("device", inner);
            }
            // Decode OS
            if let Some(os) = self.0.os.clone() {
                let inner = PyDict::new_bound(py);
                inner.set_item("name", os.name);
                if let Some(family) = os.family {
                    inner.set_item("family", family);
                }
                if let Some(platform) = os.platform {
                    inner.set_item("platform", platform);
                }

                dict.set_item("os", inner);
            }
            dict.as_any().extract()
        })
    }
}

/// Formats the sum of two numbers as string.
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
