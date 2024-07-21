use pyo3::{prelude::*, types::PyDict};

fn set_optional(dict: &Bound<PyDict>, key: &str, optional: &Option<String>) -> Result<(), PyErr> {
    match optional {
        Some(value) => dict.set_item(key, value),
        None => Ok(()),
    }
}

pub struct BotWrapper(pub rust_device_detector::device_detector::Bot);

impl BotWrapper {
    pub fn to_object(&self, py: Python) -> PyResult<PyObject> {
        let dict = PyDict::new_bound(py);
        dict.set_item("name", self.0.name.clone())?;
        set_optional(&dict, "category", &self.0.category)?;
        set_optional(&dict, "url", &self.0.url)?;
        // Decode BotProducer
        if let Some(producer) = self.0.producer.clone() {
            let inner = PyDict::new_bound(py);
            set_optional(&inner, "name", &producer.name)?;
            set_optional(&inner, "url", &producer.url)?;
            dict.set_item("producer", inner)?;
        }
        dict.as_any().extract()
    }
}

pub struct DeviceWrapper(pub rust_device_detector::device_detector::KnownDevice);

impl DeviceWrapper {
    pub fn to_object(&self, py: Python) -> PyResult<PyObject> {
        let dict = PyDict::new_bound(py);
        // Decode Client
        if let Some(client) = self.0.client.clone() {
            let inner = PyDict::new_bound(py);

            inner.set_item("name", client.name)?;
            inner.set_item("type", client.r#type.as_str())?;
            set_optional(&inner, "version", &client.version)?;
            set_optional(&inner, "engine", &client.engine)?;
            set_optional(&inner, "engine_version", &client.engine_version)?;

            dict.set_item("client", inner)?;
        }
        // Decode Device
        if let Some(device) = self.0.device.clone() {
            let inner = PyDict::new_bound(py);

            set_optional(&inner, "brand", &device.brand)?;
            set_optional(&inner, "model", &device.model)?;

            dict.set_item("device", inner)?;
        }
        // Decode OS
        if let Some(os) = self.0.os.clone() {
            let inner = PyDict::new_bound(py);
            inner.set_item("name", os.name)?;
            set_optional(&inner, "family", &os.family)?;
            set_optional(&inner, "platform", &os.platform)?;

            dict.set_item("os", inner)?;
        }
        dict.as_any().extract()
    }
}
