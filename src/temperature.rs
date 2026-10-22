use material_colors::temperature::TemperatureCache;
use pyo3::{pyclass, pymethods};

use crate::hct::PyHct;

#[pyclass(name = "TemperatureCache")]
pub struct PyTemperatureCache(pub TemperatureCache);

#[pymethods]
impl PyTemperatureCache {
    #[new]
    fn new(input: &PyHct) -> Self {
        PyTemperatureCache(TemperatureCache::new(input.0))
    }

    fn analogous(&self) -> [PyHct; 5] {
        self.0.analogous().map(PyHct)
    }

    fn complement(&mut self) -> PyHct {
        PyHct(self.0.complement())
    }

    fn relative_temperature(&self, temp: f64) -> f64 {
        self.0.relative_temperature(temp)
    }
}
