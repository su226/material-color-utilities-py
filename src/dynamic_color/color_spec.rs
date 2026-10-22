use material_colors::dynamic_color::{ColorSpec, SpecVersion, color_spec};
use pyo3::prelude::*;

use crate::{
    dynamic_color::{PyDynamicColor, dynamic_scheme::PyDynamicScheme, variant::PyVariant},
    hct::PyHct,
    palette::tonal::PyTonalPalette,
};

#[pyclass(name = "SpecVersion", from_py_object)]
#[derive(Clone, Copy)]
pub enum PySpecVersion {
    Spec2021,
    Spec2025,
    Spec2026,
}

impl From<PySpecVersion> for SpecVersion {
    fn from(version: PySpecVersion) -> Self {
        match version {
            PySpecVersion::Spec2021 => Self::Spec2021,
            PySpecVersion::Spec2025 => Self::Spec2025,
            PySpecVersion::Spec2026 => Self::Spec2026,
        }
    }
}

impl From<SpecVersion> for PySpecVersion {
    fn from(version: SpecVersion) -> Self {
        match version {
            SpecVersion::Spec2021 => Self::Spec2021,
            SpecVersion::Spec2025 => Self::Spec2025,
            SpecVersion::Spec2026 => Self::Spec2026,
        }
    }
}

/// The color definitions and palette algorithms of one spec version.
///
/// Normally a [`PyDynamicScheme`] resolves colors through its own spec version.
/// This class exposes the spec directly, which is useful when querying a color
/// definition independently of a scheme.
#[pyclass(name = "ColorSpec", subclass)]
#[derive(Clone, Copy)]
pub struct PyColorSpec {
    pub(crate) version: SpecVersion,
}

impl PyColorSpec {
    fn spec(&self) -> &'static dyn ColorSpec {
        color_spec(self.version)
    }
}

#[pymethods]
impl PyColorSpec {
    #[new]
    fn new(version: PySpecVersion) -> Self {
        Self {
            version: version.into(),
        }
    }

    #[getter]
    fn get_spec_version(&self) -> PySpecVersion {
        self.version.into()
    }

    /// Surface a color should contrast with: bright in dark mode, dim in
    /// light mode.
    fn highest_surface(&self, scheme: &PyDynamicScheme) -> PyDynamicColor {
        PyDynamicColor::from_dynamic(self.spec().highest_surface(&scheme.0))
            .expect("highest_surface is always a material role")
    }

    /// Resolves `color` using this spec's algorithm.
    fn get_hct(&self, scheme: &PyDynamicScheme, color: &PyDynamicColor) -> PyHct {
        PyHct(self.spec().get_hct(&scheme.0, color.as_dynamic()))
    }

    /// Resolves the tone of `color` using this spec's algorithm.
    fn get_tone(&self, scheme: &PyDynamicScheme, color: &PyDynamicColor) -> f64 {
        self.spec().get_tone(&scheme.0, color.as_dynamic())
    }

    fn get_primary_palette(
        &self,
        variant: PyVariant,
        source: &PyHct,
        is_dark: bool,
        platform: crate::dynamic_color::dynamic_scheme::PyPlatform,
        contrast_level: f64,
    ) -> Option<PyTonalPalette> {
        self.spec()
            .get_primary_palette(
                variant.into(),
                source.0,
                is_dark,
                platform.into(),
                contrast_level,
            )
            .map(PyTonalPalette)
    }

    fn get_secondary_palette(
        &self,
        variant: PyVariant,
        source: &PyHct,
        is_dark: bool,
        platform: crate::dynamic_color::dynamic_scheme::PyPlatform,
        contrast_level: f64,
    ) -> Option<PyTonalPalette> {
        self.spec()
            .get_secondary_palette(
                variant.into(),
                source.0,
                is_dark,
                platform.into(),
                contrast_level,
            )
            .map(PyTonalPalette)
    }

    fn get_tertiary_palette(
        &self,
        variant: PyVariant,
        source: &PyHct,
        is_dark: bool,
        platform: crate::dynamic_color::dynamic_scheme::PyPlatform,
        contrast_level: f64,
    ) -> Option<PyTonalPalette> {
        self.spec()
            .get_tertiary_palette(
                variant.into(),
                source.0,
                is_dark,
                platform.into(),
                contrast_level,
            )
            .map(PyTonalPalette)
    }

    fn get_neutral_palette(
        &self,
        variant: PyVariant,
        source: &PyHct,
        is_dark: bool,
        platform: crate::dynamic_color::dynamic_scheme::PyPlatform,
        contrast_level: f64,
    ) -> Option<PyTonalPalette> {
        self.spec()
            .get_neutral_palette(
                variant.into(),
                source.0,
                is_dark,
                platform.into(),
                contrast_level,
            )
            .map(PyTonalPalette)
    }

    fn get_neutral_variant_palette(
        &self,
        variant: PyVariant,
        source: &PyHct,
        is_dark: bool,
        platform: crate::dynamic_color::dynamic_scheme::PyPlatform,
        contrast_level: f64,
    ) -> Option<PyTonalPalette> {
        self.spec()
            .get_neutral_variant_palette(
                variant.into(),
                source.0,
                is_dark,
                platform.into(),
                contrast_level,
            )
            .map(PyTonalPalette)
    }

    fn get_error_palette(
        &self,
        variant: PyVariant,
        source: &PyHct,
        is_dark: bool,
        platform: crate::dynamic_color::dynamic_scheme::PyPlatform,
        contrast_level: f64,
    ) -> Option<PyTonalPalette> {
        self.spec()
            .get_error_palette(
                variant.into(),
                source.0,
                is_dark,
                platform.into(),
                contrast_level,
            )
            .map(PyTonalPalette)
    }
}

/// Generates the Python wrapper for one concrete spec version.
macro_rules! py_color_spec_version {
    ($struct:ident, $name:literal, $version:ident) => {
        #[pyclass(name = $name, extends = $crate::dynamic_color::color_spec::PyColorSpec)]
        pub struct $struct;

        #[pymethods]
        impl $struct {
            #[new]
            fn new() -> (Self, $crate::dynamic_color::color_spec::PyColorSpec) {
                (
                    Self,
                    $crate::dynamic_color::color_spec::PyColorSpec {
                        version: material_colors::dynamic_color::SpecVersion::$version,
                    },
                )
            }
        }
    };
}

pub(crate) use py_color_spec_version;
