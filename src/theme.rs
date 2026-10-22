use material_colors::theme::{Palettes, Schemes, Theme, ThemeBuilder};
use pyo3::{exceptions::PyValueError, prelude::*};

use crate::{
    color::{PyRgb, rgb_from_py, rgb_to_py},
    dynamic_color::{color_spec::PySpecVersion, dynamic_scheme::PyPlatform, variant::PyVariant},
    palette::tonal::PyTonalPalette,
    scheme::PyScheme,
};

#[pyclass(name = "Schemes")]
pub struct PySchemes(pub Schemes);

#[pymethods]
impl PySchemes {
    #[new]
    fn new(light: &PyScheme, dark: &PyScheme) -> Self {
        Self(Schemes {
            light: light.0,
            dark: dark.0,
        })
    }

    #[getter]
    fn get_light(&self) -> PyScheme {
        PyScheme(self.0.light)
    }

    #[setter]
    fn set_light(&mut self, light: &PyScheme) {
        self.0.light = light.0;
    }

    #[getter]
    fn get_dark(&self) -> PyScheme {
        PyScheme(self.0.dark)
    }

    #[setter]
    fn set_dark(&mut self, dark: &PyScheme) {
        self.0.dark = dark.0;
    }
}

#[pyclass(name = "Palettes")]
pub struct PyPalettes(pub Palettes);

#[pymethods]
impl PyPalettes {
    #[new]
    fn new(
        primary: &PyTonalPalette,
        secondary: &PyTonalPalette,
        tertiary: &PyTonalPalette,
        neutral: &PyTonalPalette,
        neutral_variant: &PyTonalPalette,
        error: &PyTonalPalette,
    ) -> Self {
        Self(Palettes {
            primary: primary.0,
            secondary: secondary.0,
            tertiary: tertiary.0,
            neutral: neutral.0,
            neutral_variant: neutral_variant.0,
            error: error.0,
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

    #[getter]
    fn get_error(&self) -> PyTonalPalette {
        PyTonalPalette(self.0.error)
    }

    #[setter]
    fn set_error(&mut self, error: &PyTonalPalette) {
        self.0.error = error.0;
    }
}

#[pyclass(name = "Theme")]
pub struct PyTheme(pub Theme);

#[pymethods]
impl PyTheme {
    #[new]
    fn new(source: PyRgb, schemes: &PySchemes, palettes: &PyPalettes) -> Self {
        Self(Theme {
            source: rgb_from_py(source),
            schemes: schemes.0,
            palettes: palettes.0,
        })
    }

    #[getter]
    fn get_source(&self) -> PyRgb {
        rgb_to_py(self.0.source)
    }

    #[setter]
    fn set_source(&mut self, source: PyRgb) {
        self.0.source = rgb_from_py(source);
    }

    #[getter]
    fn get_schemes(&self) -> PySchemes {
        PySchemes(self.0.schemes)
    }

    #[setter]
    fn set_schemes(&mut self, schemes: &PySchemes) {
        self.0.schemes = schemes.0;
    }

    #[getter]
    fn get_palettes(&self) -> PyPalettes {
        PyPalettes(self.0.palettes)
    }

    #[setter]
    fn set_palettes(&mut self, palettes: &PyPalettes) {
        self.0.palettes = palettes.0;
    }
}

#[pyclass(name = "ThemeBuilder")]
pub struct PyThemeBuilder(pub Option<ThemeBuilder>);

#[pymethods]
impl PyThemeBuilder {
    #[new]
    fn new(rgb: PyRgb) -> Self {
        Self(Some(ThemeBuilder::with_source(rgb_from_py(rgb))))
    }

    #[setter]
    pub fn set_contrast_level(&mut self, level: f64) -> PyResult<()> {
        let builder = self
            .0
            .take()
            .ok_or_else(|| PyValueError::new_err("ThemeBuilder already used."))?
            .contrast_level(level);
        self.0 = Some(builder);
        Ok(())
    }

    #[setter]
    pub fn set_spec_version(&mut self, version: PySpecVersion) -> PyResult<()> {
        let builder = self
            .0
            .take()
            .ok_or_else(|| PyValueError::new_err("ThemeBuilder already used."))?
            .spec_version(version.into());
        self.0 = Some(builder);
        Ok(())
    }

    #[setter]
    pub fn set_platform(&mut self, platform: PyPlatform) -> PyResult<()> {
        let builder = self
            .0
            .take()
            .ok_or_else(|| PyValueError::new_err("ThemeBuilder already used."))?
            .platform(platform.into());
        self.0 = Some(builder);
        Ok(())
    }

    #[setter]
    pub fn set_variant(&mut self, variant: PyVariant) -> PyResult<()> {
        let builder = self
            .0
            .take()
            .ok_or_else(|| PyValueError::new_err("ThemeBuilder already used."))?
            .variant(variant.into());
        self.0 = Some(builder);
        Ok(())
    }

    #[setter]
    pub fn set_primary(&mut self, color: PyRgb) -> PyResult<()> {
        let builder = self
            .0
            .take()
            .ok_or_else(|| PyValueError::new_err("ThemeBuilder already used."))?
            .primary(rgb_from_py(color));
        self.0 = Some(builder);
        Ok(())
    }

    #[setter]
    pub fn set_secondary(&mut self, color: PyRgb) -> PyResult<()> {
        let builder = self
            .0
            .take()
            .ok_or_else(|| PyValueError::new_err("ThemeBuilder already used."))?
            .secondary(rgb_from_py(color));
        self.0 = Some(builder);
        Ok(())
    }

    #[setter]
    pub fn set_tertiary(&mut self, color: PyRgb) -> PyResult<()> {
        let builder = self
            .0
            .take()
            .ok_or_else(|| PyValueError::new_err("ThemeBuilder already used."))?
            .tertiary(rgb_from_py(color));
        self.0 = Some(builder);
        Ok(())
    }

    #[setter]
    pub fn set_error(&mut self, color: PyRgb) -> PyResult<()> {
        let builder = self
            .0
            .take()
            .ok_or_else(|| PyValueError::new_err("ThemeBuilder already used."))?
            .error(rgb_from_py(color));
        self.0 = Some(builder);
        Ok(())
    }

    #[setter]
    pub fn set_neutral(&mut self, color: PyRgb) -> PyResult<()> {
        let builder = self
            .0
            .take()
            .ok_or_else(|| PyValueError::new_err("ThemeBuilder already used."))?
            .neutral(rgb_from_py(color));
        self.0 = Some(builder);
        Ok(())
    }

    #[setter]
    pub fn set_neutral_variant(&mut self, color: PyRgb) -> PyResult<()> {
        let builder = self
            .0
            .take()
            .ok_or_else(|| PyValueError::new_err("ThemeBuilder already used."))?
            .neutral_variant(rgb_from_py(color));
        self.0 = Some(builder);
        Ok(())
    }

    #[setter]
    pub fn set_color_match(&mut self, enabled: bool) -> PyResult<()> {
        let builder = self
            .0
            .take()
            .ok_or_else(|| PyValueError::new_err("ThemeBuilder already used."))?
            .color_match(enabled);
        self.0 = Some(builder);
        Ok(())
    }

    fn build(&mut self) -> PyResult<PyTheme> {
        let theme = self
            .0
            .take()
            .ok_or_else(|| PyValueError::new_err("ThemeBuilder already used."))?
            .build();
        Ok(PyTheme(theme))
    }
}
