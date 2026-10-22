use material_colors::palette::{CorePalette, CorePalettes};
use pyo3::prelude::*;

use crate::{
    color::{PyRgb, rgb_from_py},
    palette::tonal::PyTonalPalette,
};

#[pyclass(name = "CorePalette")]
pub struct PyCorePalette(pub CorePalette);

#[pymethods]
impl PyCorePalette {
    #[new]
    #[pyo3(signature = (primary, secondary, tertiary, neutral, neutral_variant, error = None))]
    fn new(
        primary: &PyTonalPalette,
        secondary: &PyTonalPalette,
        tertiary: &PyTonalPalette,
        neutral: &PyTonalPalette,
        neutral_variant: &PyTonalPalette,
        error: Option<&PyTonalPalette>,
    ) -> Self {
        Self(CorePalette::new(
            primary.0,
            secondary.0,
            tertiary.0,
            neutral.0,
            neutral_variant.0,
            error.map(|x| x.0),
        ))
    }

    #[staticmethod]
    fn of(rgb: PyRgb) -> Self {
        Self(CorePalette::of(rgb_from_py(rgb)))
    }

    #[staticmethod]
    fn content_of(rgb: PyRgb) -> Self {
        Self(CorePalette::content_of(rgb_from_py(rgb)))
    }

    #[getter]
    fn get_primary(&self) -> PyTonalPalette {
        PyTonalPalette(self.0.primary)
    }

    #[setter]
    fn set_primary(&mut self, primary: &PyTonalPalette) {
        self.0.primary = primary.0;
    }

    #[getter]
    fn get_secondary(&self) -> PyTonalPalette {
        PyTonalPalette(self.0.secondary)
    }

    #[setter]
    fn set_secondary(&mut self, secondary: &PyTonalPalette) {
        self.0.secondary = secondary.0;
    }

    #[getter]
    fn get_tertiary(&self) -> PyTonalPalette {
        PyTonalPalette(self.0.tertiary)
    }

    #[setter]
    fn set_tertiary(&mut self, tertiary: &PyTonalPalette) {
        self.0.tertiary = tertiary.0;
    }

    #[getter]
    fn get_neutral(&self) -> PyTonalPalette {
        PyTonalPalette(self.0.neutral)
    }

    #[setter]
    fn set_neutral(&mut self, neutral: &PyTonalPalette) {
        self.0.neutral = neutral.0;
    }

    #[getter]
    fn get_neutral_variant(&self) -> PyTonalPalette {
        PyTonalPalette(self.0.neutral_variant)
    }

    #[setter]
    fn set_neutral_variant(&mut self, neutral_variant: &PyTonalPalette) {
        self.0.neutral_variant = neutral_variant.0;
    }

    #[getter]
    fn get_error(&self) -> PyTonalPalette {
        PyTonalPalette(self.0.error)
    }

    #[setter]
    fn set_error(&mut self, error: &PyTonalPalette) {
        self.0.error = error.0;
    }
}

#[pyclass(name = "CorePalettes")]
pub struct PyCorePalettes(pub CorePalettes);

#[pymethods]
impl PyCorePalettes {
    #[new]
    fn new(
        primary: &PyTonalPalette,
        secondary: &PyTonalPalette,
        tertiary: &PyTonalPalette,
        neutral: &PyTonalPalette,
        neutral_variant: &PyTonalPalette,
    ) -> Self {
        Self(CorePalettes {
            primary: primary.0,
            secondary: secondary.0,
            tertiary: tertiary.0,
            neutral: neutral.0,
            neutral_variant: neutral_variant.0,
        })
    }

    #[getter]
    fn get_primary(&self) -> PyTonalPalette {
        PyTonalPalette(self.0.primary)
    }

    #[setter]
    fn set_primary(&mut self, primary: &PyTonalPalette) {
        self.0.primary = primary.0;
    }

    #[getter]
    fn get_secondary(&self) -> PyTonalPalette {
        PyTonalPalette(self.0.secondary)
    }

    #[setter]
    fn set_secondary(&mut self, secondary: &PyTonalPalette) {
        self.0.secondary = secondary.0;
    }

    #[getter]
    fn get_tertiary(&self) -> PyTonalPalette {
        PyTonalPalette(self.0.tertiary)
    }

    #[setter]
    fn set_tertiary(&mut self, tertiary: &PyTonalPalette) {
        self.0.tertiary = tertiary.0;
    }

    #[getter]
    fn get_neutral(&self) -> PyTonalPalette {
        PyTonalPalette(self.0.neutral)
    }

    #[setter]
    fn set_neutral(&mut self, neutral: &PyTonalPalette) {
        self.0.neutral = neutral.0;
    }

    #[getter]
    fn get_neutral_variant(&self) -> PyTonalPalette {
        PyTonalPalette(self.0.neutral_variant)
    }

    #[setter]
    fn set_neutral_variant(&mut self, neutral_variant: &PyTonalPalette) {
        self.0.neutral_variant = neutral_variant.0;
    }
}
