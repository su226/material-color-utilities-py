use material_colors::palette::TonalPalette;
use pyo3::prelude::*;

use crate::{
    color::{PyRgb, rgb_to_py},
    dynamic_color::variant::PyVariant,
    hct::PyHct,
    palette::PyPalette,
};

#[pyclass(name = "TonalPalette")]
pub struct PyTonalPalette(pub TonalPalette);

#[pymethods]
impl PyTonalPalette {
    #[getter]
    fn get_hue(&self) -> f64 {
        self.0.hue()
    }

    #[getter]
    fn get_chroma(&self) -> f64 {
        self.0.chroma()
    }

    #[getter]
    fn get_key_color(&self) -> PyHct {
        PyHct(self.0.key_color())
    }

    #[new]
    fn new(hct: &PyHct) -> Self {
        Self(TonalPalette::from_hct(hct.0))
    }

    #[staticmethod]
    fn by_variant(source_hct: &PyHct, scheme: PyVariant, variant: PyPalette) -> Self {
        Self(TonalPalette::by_variant(
            &source_hct.0,
            &scheme.into(),
            &variant.into(),
        ))
    }

    #[staticmethod]
    pub fn of(hue: f64, chroma: f64) -> Self {
        Self(TonalPalette::of(hue, chroma))
    }

    fn tone(&self, tone: i32) -> PyRgb {
        rgb_to_py(self.0.tone(tone))
    }

    fn get_hct(&self, tone: f64) -> PyHct {
        PyHct(self.0.get_hct(tone))
    }
}
