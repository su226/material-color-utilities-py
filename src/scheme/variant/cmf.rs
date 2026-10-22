use material_colors::scheme::variant::SchemeCmf;
use pyo3::prelude::*;

use crate::{
    dynamic_color::{
        dynamic_scheme::{PyDynamicScheme, PyPlatform},
        variant::PyVariant,
    },
    hct::PyHct,
    palette::{PyPalette, tonal::PyTonalPalette},
    scheme::PyScheme,
};

/// A dynamic color theme with up to 2 source colors.
///
/// Only defined for spec 2026, so there is no spec parameter.
#[pyclass(name = "SchemeCmf")]
pub struct PySchemeCmf(pub SchemeCmf);

#[pymethods]
impl PySchemeCmf {
    /// One source color, phone.
    #[new]
    #[pyo3(signature = (source_color_hct, is_dark, contrast_level = None))]
    pub fn new(source_color_hct: &PyHct, is_dark: bool, contrast_level: Option<f64>) -> Self {
        Self(SchemeCmf::new(source_color_hct.0, is_dark, contrast_level))
    }

    /// One or two source colors.
    #[staticmethod]
    #[pyo3(signature = (source_color_hct, secondary_source_color_hct, is_dark, contrast_level, platform))]
    pub fn with_sources(
        source_color_hct: &PyHct,
        secondary_source_color_hct: Option<&PyHct>,
        is_dark: bool,
        contrast_level: Option<f64>,
        platform: PyPlatform,
    ) -> Self {
        Self(SchemeCmf::with_sources(
            source_color_hct.0,
            secondary_source_color_hct.map(|hct| hct.0),
            is_dark,
            contrast_level,
            platform.into(),
        ))
    }

    /// One palette for a single source color.
    #[staticmethod]
    pub fn palette(source_color_hct: &PyHct, palette: PyPalette) -> PyTonalPalette {
        PyTonalPalette(SchemeCmf::palette(&source_color_hct.0, &palette.into()))
    }

    /// One palette for one or two source colors.
    #[staticmethod]
    pub fn palette_with_secondary(
        source_color_hct: &PyHct,
        secondary_source_color_hct: Option<&PyHct>,
        palette: PyPalette,
    ) -> PyTonalPalette {
        PyTonalPalette(SchemeCmf::palette_with_secondary(
            &source_color_hct.0,
            secondary_source_color_hct.map(|hct| &hct.0),
            &palette.into(),
        ))
    }

    /// The underlying dynamic scheme.
    #[getter]
    fn get_scheme(&self) -> PyDynamicScheme {
        PyDynamicScheme(self.0.scheme.clone())
    }

    /// The variant of this scheme.
    #[getter]
    fn get_variant(&self) -> PyVariant {
        PyVariant::Cmf
    }

    /// Converts this scheme into a plain 49-color `Scheme`.
    fn to_scheme(&self) -> PyScheme {
        PyScheme(material_colors::scheme::Scheme::from(self.0.scheme.clone()))
    }

    fn __repr__(&self) -> String {
        format!(
            "SchemeCmf({:?}, is_dark={})",
            self.0.scheme.variant, self.0.scheme.is_dark
        )
    }
}
