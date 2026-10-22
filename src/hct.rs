use material_colors::{color::Rgb, hct::Hct};
use pyo3::prelude::*;

use crate::color::{PyRgb, rgb_from_py, rgb_to_py};

pub mod cam16;
pub mod solver;
pub mod viewing_conditions;

#[pyclass(name = "Hct")]
#[derive(Clone, Copy)]
pub struct PyHct(pub Hct);

#[pymethods]
impl PyHct {
    #[new]
    fn new(hue: f64, chroma: f64, tone: f64) -> Self {
        Self(Hct::from(hue, chroma, tone))
    }

    #[staticmethod]
    fn from_rgb(rgb: PyRgb) -> Self {
        Self(rgb_from_py(rgb).into())
    }

    #[getter]
    fn get_hue(&self) -> f64 {
        self.0.get_hue()
    }

    #[setter]
    fn set_hue(&mut self, hue: f64) {
        self.0.set_hue(hue);
    }

    #[getter]
    fn get_chroma(&self) -> f64 {
        self.0.get_chroma()
    }

    #[setter]
    fn set_chroma(&mut self, chroma: f64) {
        self.0.set_chroma(chroma);
    }

    #[getter]
    fn get_tone(&self) -> f64 {
        self.0.get_tone()
    }

    #[setter]
    fn set_tone(&mut self, tone: f64) {
        self.0.set_tone(tone);
    }

    // PyO3 can only use &self or &mut self
    #[allow(clippy::wrong_self_convention)]
    fn to_rgb(&self) -> PyRgb {
        rgb_to_py(Rgb::from(self.0))
    }
}
