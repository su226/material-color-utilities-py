use material_colors::scheme::Scheme;
use pyo3::prelude::*;

use crate::color::{PyRgb, rgb_from_py, rgb_to_py};
use crate::dynamic_color::dynamic_scheme::PyDynamicScheme;

pub mod variant;

#[pyclass(name = "Scheme")]
#[derive(Clone, Copy)]
pub struct PyScheme(pub Scheme);

#[pymethods]
impl PyScheme {
    #[new]
    pub fn new(
        primary: PyRgb,
        on_primary: PyRgb,
        primary_container: PyRgb,
        on_primary_container: PyRgb,
        inverse_primary: PyRgb,
        primary_fixed: PyRgb,
        primary_fixed_dim: PyRgb,
        on_primary_fixed: PyRgb,
        on_primary_fixed_variant: PyRgb,
        secondary: PyRgb,
        on_secondary: PyRgb,
        secondary_container: PyRgb,
        on_secondary_container: PyRgb,
        secondary_fixed: PyRgb,
        secondary_fixed_dim: PyRgb,
        on_secondary_fixed: PyRgb,
        on_secondary_fixed_variant: PyRgb,
        tertiary: PyRgb,
        on_tertiary: PyRgb,
        tertiary_container: PyRgb,
        on_tertiary_container: PyRgb,
        tertiary_fixed: PyRgb,
        tertiary_fixed_dim: PyRgb,
        on_tertiary_fixed: PyRgb,
        on_tertiary_fixed_variant: PyRgb,
        error: PyRgb,
        on_error: PyRgb,
        error_container: PyRgb,
        on_error_container: PyRgb,
        surface_dim: PyRgb,
        surface: PyRgb,
        surface_tint: PyRgb,
        surface_bright: PyRgb,
        surface_container_lowest: PyRgb,
        surface_container_low: PyRgb,
        surface_container: PyRgb,
        surface_container_high: PyRgb,
        surface_container_highest: PyRgb,
        on_surface: PyRgb,
        on_surface_variant: PyRgb,
        outline: PyRgb,
        outline_variant: PyRgb,
        inverse_surface: PyRgb,
        inverse_on_surface: PyRgb,
        surface_variant: PyRgb,
        background: PyRgb,
        on_background: PyRgb,
        shadow: PyRgb,
        scrim: PyRgb,
    ) -> Self {
        Self(Scheme::new(
            rgb_from_py(primary),
            rgb_from_py(on_primary),
            rgb_from_py(primary_container),
            rgb_from_py(on_primary_container),
            rgb_from_py(inverse_primary),
            rgb_from_py(primary_fixed),
            rgb_from_py(primary_fixed_dim),
            rgb_from_py(on_primary_fixed),
            rgb_from_py(on_primary_fixed_variant),
            rgb_from_py(secondary),
            rgb_from_py(on_secondary),
            rgb_from_py(secondary_container),
            rgb_from_py(on_secondary_container),
            rgb_from_py(secondary_fixed),
            rgb_from_py(secondary_fixed_dim),
            rgb_from_py(on_secondary_fixed),
            rgb_from_py(on_secondary_fixed_variant),
            rgb_from_py(tertiary),
            rgb_from_py(on_tertiary),
            rgb_from_py(tertiary_container),
            rgb_from_py(on_tertiary_container),
            rgb_from_py(tertiary_fixed),
            rgb_from_py(tertiary_fixed_dim),
            rgb_from_py(on_tertiary_fixed),
            rgb_from_py(on_tertiary_fixed_variant),
            rgb_from_py(error),
            rgb_from_py(on_error),
            rgb_from_py(error_container),
            rgb_from_py(on_error_container),
            rgb_from_py(surface_dim),
            rgb_from_py(surface),
            rgb_from_py(surface_tint),
            rgb_from_py(surface_bright),
            rgb_from_py(surface_container_lowest),
            rgb_from_py(surface_container_low),
            rgb_from_py(surface_container),
            rgb_from_py(surface_container_high),
            rgb_from_py(surface_container_highest),
            rgb_from_py(on_surface),
            rgb_from_py(on_surface_variant),
            rgb_from_py(outline),
            rgb_from_py(outline_variant),
            rgb_from_py(inverse_surface),
            rgb_from_py(inverse_on_surface),
            rgb_from_py(surface_variant),
            rgb_from_py(background),
            rgb_from_py(on_background),
            rgb_from_py(shadow),
            rgb_from_py(scrim),
        ))
    }

    /// Converts a dynamic scheme into a plain 49-color scheme.
    #[staticmethod]
    fn from_dynamic_scheme(scheme: &PyDynamicScheme) -> Self {
        Self(Scheme::from(scheme.0.clone()))
    }

    #[getter]
    fn get_primary(&self) -> PyRgb {
        rgb_to_py(self.0.primary)
    }

    #[setter]
    fn set_primary(&mut self, primary: PyRgb) {
        self.0.primary = rgb_from_py(primary);
    }

    #[getter]
    fn get_on_primary(&self) -> PyRgb {
        rgb_to_py(self.0.on_primary)
    }

    #[setter]
    fn set_on_primary(&mut self, on_primary: PyRgb) {
        self.0.on_primary = rgb_from_py(on_primary);
    }

    #[getter]
    fn get_primary_container(&self) -> PyRgb {
        rgb_to_py(self.0.primary_container)
    }

    #[setter]
    fn set_primary_container(&mut self, primary_container: PyRgb) {
        self.0.primary_container = rgb_from_py(primary_container);
    }

    #[getter]
    fn get_on_primary_container(&self) -> PyRgb {
        rgb_to_py(self.0.on_primary_container)
    }

    #[setter]
    fn set_on_primary_container(&mut self, on_primary_container: PyRgb) {
        self.0.on_primary_container = rgb_from_py(on_primary_container);
    }

    #[getter]
    fn get_inverse_primary(&self) -> PyRgb {
        rgb_to_py(self.0.inverse_primary)
    }

    #[setter]
    fn set_inverse_primary(&mut self, inverse_primary: PyRgb) {
        self.0.inverse_primary = rgb_from_py(inverse_primary);
    }

    #[getter]
    fn get_primary_fixed(&self) -> PyRgb {
        rgb_to_py(self.0.primary_fixed)
    }

    #[setter]
    fn set_primary_fixed(&mut self, primary_fixed: PyRgb) {
        self.0.primary_fixed = rgb_from_py(primary_fixed);
    }

    #[getter]
    fn get_primary_fixed_dim(&self) -> PyRgb {
        rgb_to_py(self.0.primary_fixed_dim)
    }

    #[setter]
    fn set_primary_fixed_dim(&mut self, primary_fixed_dim: PyRgb) {
        self.0.primary_fixed_dim = rgb_from_py(primary_fixed_dim);
    }

    #[getter]
    fn get_on_primary_fixed(&self) -> PyRgb {
        rgb_to_py(self.0.on_primary_fixed)
    }

    #[setter]
    fn set_on_primary_fixed(&mut self, on_primary_fixed: PyRgb) {
        self.0.on_primary_fixed = rgb_from_py(on_primary_fixed);
    }

    #[getter]
    fn get_on_primary_fixed_variant(&self) -> PyRgb {
        rgb_to_py(self.0.on_primary_fixed_variant)
    }

    #[setter]
    fn set_on_primary_fixed_variant(&mut self, on_primary_fixed_variant: PyRgb) {
        self.0.on_primary_fixed_variant = rgb_from_py(on_primary_fixed_variant);
    }

    #[getter]
    fn get_secondary(&self) -> PyRgb {
        rgb_to_py(self.0.secondary)
    }

    #[setter]
    fn set_secondary(&mut self, secondary: PyRgb) {
        self.0.secondary = rgb_from_py(secondary);
    }

    #[getter]
    fn get_on_secondary(&self) -> PyRgb {
        rgb_to_py(self.0.on_secondary)
    }

    #[setter]
    fn set_on_secondary(&mut self, on_secondary: PyRgb) {
        self.0.on_secondary = rgb_from_py(on_secondary);
    }

    #[getter]
    fn get_secondary_container(&self) -> PyRgb {
        rgb_to_py(self.0.secondary_container)
    }

    #[setter]
    fn set_secondary_container(&mut self, secondary_container: PyRgb) {
        self.0.secondary_container = rgb_from_py(secondary_container);
    }

    #[getter]
    fn get_on_secondary_container(&self) -> PyRgb {
        rgb_to_py(self.0.on_secondary_container)
    }

    #[setter]
    fn set_on_secondary_container(&mut self, on_secondary_container: PyRgb) {
        self.0.on_secondary_container = rgb_from_py(on_secondary_container);
    }

    #[getter]
    fn get_secondary_fixed(&self) -> PyRgb {
        rgb_to_py(self.0.secondary_fixed)
    }

    #[setter]
    fn set_secondary_fixed(&mut self, secondary_fixed: PyRgb) {
        self.0.secondary_fixed = rgb_from_py(secondary_fixed);
    }

    #[getter]
    fn get_secondary_fixed_dim(&self) -> PyRgb {
        rgb_to_py(self.0.secondary_fixed_dim)
    }

    #[setter]
    fn set_secondary_fixed_dim(&mut self, secondary_fixed_dim: PyRgb) {
        self.0.secondary_fixed_dim = rgb_from_py(secondary_fixed_dim);
    }

    #[getter]
    fn get_on_secondary_fixed(&self) -> PyRgb {
        rgb_to_py(self.0.on_secondary_fixed)
    }

    #[setter]
    fn set_on_secondary_fixed(&mut self, on_secondary_fixed: PyRgb) {
        self.0.on_secondary_fixed = rgb_from_py(on_secondary_fixed);
    }

    #[getter]
    fn get_on_secondary_fixed_variant(&self) -> PyRgb {
        rgb_to_py(self.0.on_secondary_fixed_variant)
    }

    #[setter]
    fn set_on_secondary_fixed_variant(&mut self, on_secondary_fixed_variant: PyRgb) {
        self.0.on_secondary_fixed_variant = rgb_from_py(on_secondary_fixed_variant);
    }

    #[getter]
    fn get_tertiary(&self) -> PyRgb {
        rgb_to_py(self.0.tertiary)
    }

    #[setter]
    fn set_tertiary(&mut self, tertiary: PyRgb) {
        self.0.tertiary = rgb_from_py(tertiary);
    }

    #[getter]
    fn get_on_tertiary(&self) -> PyRgb {
        rgb_to_py(self.0.on_tertiary)
    }

    #[setter]
    fn set_on_tertiary(&mut self, on_tertiary: PyRgb) {
        self.0.on_tertiary = rgb_from_py(on_tertiary);
    }

    #[getter]
    fn get_tertiary_container(&self) -> PyRgb {
        rgb_to_py(self.0.tertiary_container)
    }

    #[setter]
    fn set_tertiary_container(&mut self, tertiary_container: PyRgb) {
        self.0.tertiary_container = rgb_from_py(tertiary_container);
    }

    #[getter]
    fn get_on_tertiary_container(&self) -> PyRgb {
        rgb_to_py(self.0.on_tertiary_container)
    }

    #[setter]
    fn set_on_tertiary_container(&mut self, on_tertiary_container: PyRgb) {
        self.0.on_tertiary_container = rgb_from_py(on_tertiary_container);
    }

    #[getter]
    fn get_tertiary_fixed(&self) -> PyRgb {
        rgb_to_py(self.0.tertiary_fixed)
    }

    #[setter]
    fn set_tertiary_fixed(&mut self, tertiary_fixed: PyRgb) {
        self.0.tertiary_fixed = rgb_from_py(tertiary_fixed);
    }

    #[getter]
    fn get_tertiary_fixed_dim(&self) -> PyRgb {
        rgb_to_py(self.0.tertiary_fixed_dim)
    }

    #[setter]
    fn set_tertiary_fixed_dim(&mut self, tertiary_fixed_dim: PyRgb) {
        self.0.tertiary_fixed_dim = rgb_from_py(tertiary_fixed_dim);
    }

    #[getter]
    fn get_on_tertiary_fixed(&self) -> PyRgb {
        rgb_to_py(self.0.on_tertiary_fixed)
    }

    #[setter]
    fn set_on_tertiary_fixed(&mut self, on_tertiary_fixed: PyRgb) {
        self.0.on_tertiary_fixed = rgb_from_py(on_tertiary_fixed);
    }

    #[getter]
    fn get_on_tertiary_fixed_variant(&self) -> PyRgb {
        rgb_to_py(self.0.on_tertiary_fixed_variant)
    }

    #[setter]
    fn set_on_tertiary_fixed_variant(&mut self, on_tertiary_fixed_variant: PyRgb) {
        self.0.on_tertiary_fixed_variant = rgb_from_py(on_tertiary_fixed_variant);
    }

    #[getter]
    fn get_error(&self) -> PyRgb {
        rgb_to_py(self.0.error)
    }

    #[setter]
    fn set_error(&mut self, error: PyRgb) {
        self.0.error = rgb_from_py(error);
    }

    #[getter]
    fn get_on_error(&self) -> PyRgb {
        rgb_to_py(self.0.on_error)
    }

    #[setter]
    fn set_on_error(&mut self, on_error: PyRgb) {
        self.0.on_error = rgb_from_py(on_error);
    }

    #[getter]
    fn get_error_container(&self) -> PyRgb {
        rgb_to_py(self.0.error_container)
    }

    #[setter]
    fn set_error_container(&mut self, error_container: PyRgb) {
        self.0.error_container = rgb_from_py(error_container);
    }

    #[getter]
    fn get_on_error_container(&self) -> PyRgb {
        rgb_to_py(self.0.on_error_container)
    }

    #[setter]
    fn set_on_error_container(&mut self, on_error_container: PyRgb) {
        self.0.on_error_container = rgb_from_py(on_error_container);
    }

    #[getter]
    fn get_surface_dim(&self) -> PyRgb {
        rgb_to_py(self.0.surface_dim)
    }

    #[setter]
    fn set_surface_dim(&mut self, surface_dim: PyRgb) {
        self.0.surface_dim = rgb_from_py(surface_dim);
    }

    #[getter]
    fn get_surface(&self) -> PyRgb {
        rgb_to_py(self.0.surface)
    }

    #[setter]
    fn set_surface(&mut self, surface: PyRgb) {
        self.0.surface = rgb_from_py(surface);
    }

    #[getter]
    fn get_surface_tint(&self) -> PyRgb {
        rgb_to_py(self.0.surface_tint)
    }

    #[setter]
    fn set_surface_tint(&mut self, surface_tint: PyRgb) {
        self.0.surface_tint = rgb_from_py(surface_tint);
    }

    #[getter]
    fn get_surface_bright(&self) -> PyRgb {
        rgb_to_py(self.0.surface_bright)
    }

    #[setter]
    fn set_surface_bright(&mut self, surface_bright: PyRgb) {
        self.0.surface_bright = rgb_from_py(surface_bright);
    }

    #[getter]
    fn get_surface_container_lowest(&self) -> PyRgb {
        rgb_to_py(self.0.surface_container_lowest)
    }

    #[setter]
    fn set_surface_container_lowest(&mut self, surface_container_lowest: PyRgb) {
        self.0.surface_container_lowest = rgb_from_py(surface_container_lowest);
    }

    #[getter]
    fn get_surface_container_low(&self) -> PyRgb {
        rgb_to_py(self.0.surface_container_low)
    }

    #[setter]
    fn set_surface_container_low(&mut self, surface_container_low: PyRgb) {
        self.0.surface_container_low = rgb_from_py(surface_container_low);
    }

    #[getter]
    fn get_surface_container(&self) -> PyRgb {
        rgb_to_py(self.0.surface_container)
    }

    #[setter]
    fn set_surface_container(&mut self, surface_container: PyRgb) {
        self.0.surface_container = rgb_from_py(surface_container);
    }

    #[getter]
    fn get_surface_container_high(&self) -> PyRgb {
        rgb_to_py(self.0.surface_container_high)
    }

    #[setter]
    fn set_surface_container_high(&mut self, surface_container_high: PyRgb) {
        self.0.surface_container_high = rgb_from_py(surface_container_high);
    }

    #[getter]
    fn get_surface_container_highest(&self) -> PyRgb {
        rgb_to_py(self.0.surface_container_highest)
    }

    #[setter]
    fn set_surface_container_highest(&mut self, surface_container_highest: PyRgb) {
        self.0.surface_container_highest = rgb_from_py(surface_container_highest);
    }

    #[getter]
    fn get_on_surface(&self) -> PyRgb {
        rgb_to_py(self.0.on_surface)
    }

    #[setter]
    fn set_on_surface(&mut self, on_surface: PyRgb) {
        self.0.on_surface = rgb_from_py(on_surface);
    }

    #[getter]
    fn get_on_surface_variant(&self) -> PyRgb {
        rgb_to_py(self.0.on_surface_variant)
    }

    #[setter]
    fn set_on_surface_variant(&mut self, on_surface_variant: PyRgb) {
        self.0.on_surface_variant = rgb_from_py(on_surface_variant);
    }

    #[getter]
    fn get_outline(&self) -> PyRgb {
        rgb_to_py(self.0.outline)
    }

    #[setter]
    fn set_outline(&mut self, outline: PyRgb) {
        self.0.outline = rgb_from_py(outline);
    }

    #[getter]
    fn get_outline_variant(&self) -> PyRgb {
        rgb_to_py(self.0.outline_variant)
    }

    #[setter]
    fn set_outline_variant(&mut self, outline_variant: PyRgb) {
        self.0.outline_variant = rgb_from_py(outline_variant);
    }

    #[getter]
    fn get_inverse_surface(&self) -> PyRgb {
        rgb_to_py(self.0.inverse_surface)
    }

    #[setter]
    fn set_inverse_surface(&mut self, inverse_surface: PyRgb) {
        self.0.inverse_surface = rgb_from_py(inverse_surface);
    }

    #[getter]
    fn get_inverse_on_surface(&self) -> PyRgb {
        rgb_to_py(self.0.inverse_on_surface)
    }

    #[setter]
    fn set_inverse_on_surface(&mut self, inverse_on_surface: PyRgb) {
        self.0.inverse_on_surface = rgb_from_py(inverse_on_surface);
    }

    #[getter]
    fn get_surface_variant(&self) -> PyRgb {
        rgb_to_py(self.0.surface_variant)
    }

    #[setter]
    fn set_surface_variant(&mut self, surface_variant: PyRgb) {
        self.0.surface_variant = rgb_from_py(surface_variant);
    }

    #[getter]
    fn get_background(&self) -> PyRgb {
        rgb_to_py(self.0.background)
    }

    #[setter]
    fn set_background(&mut self, background: PyRgb) {
        self.0.background = rgb_from_py(background);
    }

    #[getter]
    fn get_on_background(&self) -> PyRgb {
        rgb_to_py(self.0.on_background)
    }

    #[setter]
    fn set_on_background(&mut self, on_background: PyRgb) {
        self.0.on_background = rgb_from_py(on_background);
    }

    #[getter]
    fn get_shadow(&self) -> PyRgb {
        rgb_to_py(self.0.shadow)
    }

    #[setter]
    fn set_shadow(&mut self, shadow: PyRgb) {
        self.0.shadow = rgb_from_py(shadow);
    }

    #[getter]
    fn get_scrim(&self) -> PyRgb {
        rgb_to_py(self.0.scrim)
    }

    #[setter]
    fn set_scrim(&mut self, scrim: PyRgb) {
        self.0.scrim = rgb_from_py(scrim);
    }
}
