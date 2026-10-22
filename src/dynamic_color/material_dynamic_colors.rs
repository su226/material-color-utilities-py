//! Tokens, or named colors, in the Material Design system.

use material_colors::dynamic_color::{MaterialDynamicColors, Role};
use pyo3::prelude::*;

use crate::dynamic_color::{PyDynamicColor, dynamic_scheme::PyDynamicScheme};

/// Tokens, or named colors, in the Material Design system.
#[pyclass(name = "MaterialDynamicColors")]
#[derive(Clone, Copy)]
pub struct PyMaterialDynamicColors;

#[pymethods]
impl PyMaterialDynamicColors {
    /// The surface a color should contrast with: bright in dark mode, dim in
    /// light mode.
    #[staticmethod]
    fn highest_surface(scheme: &PyDynamicScheme) -> PyDynamicColor {
        PyDynamicColor::from_dynamic(MaterialDynamicColors::highest_surface(&scheme.0))
            .expect("highest_surface is always a material role")
    }

    /// All tokens, in `Role` order.
    #[staticmethod]
    fn all() -> Vec<PyDynamicColor> {
        MaterialDynamicColors::ALL
            .iter()
            .copied()
            .map(PyDynamicColor::material)
            .collect()
    }

    #[staticmethod]
    fn primary_palette_key_color() -> PyDynamicColor {
        PyDynamicColor::material(Role::PrimaryPaletteKeyColor)
    }

    #[staticmethod]
    fn secondary_palette_key_color() -> PyDynamicColor {
        PyDynamicColor::material(Role::SecondaryPaletteKeyColor)
    }

    #[staticmethod]
    fn tertiary_palette_key_color() -> PyDynamicColor {
        PyDynamicColor::material(Role::TertiaryPaletteKeyColor)
    }

    #[staticmethod]
    fn neutral_palette_key_color() -> PyDynamicColor {
        PyDynamicColor::material(Role::NeutralPaletteKeyColor)
    }

    #[staticmethod]
    fn neutral_variant_palette_key_color() -> PyDynamicColor {
        PyDynamicColor::material(Role::NeutralVariantPaletteKeyColor)
    }

    #[staticmethod]
    fn error_palette_key_color() -> PyDynamicColor {
        PyDynamicColor::material(Role::ErrorPaletteKeyColor)
    }

    #[staticmethod]
    fn background() -> PyDynamicColor {
        PyDynamicColor::material(Role::Background)
    }

    #[staticmethod]
    fn on_background() -> PyDynamicColor {
        PyDynamicColor::material(Role::OnBackground)
    }

    #[staticmethod]
    fn surface() -> PyDynamicColor {
        PyDynamicColor::material(Role::Surface)
    }

    #[staticmethod]
    fn surface_dim() -> PyDynamicColor {
        PyDynamicColor::material(Role::SurfaceDim)
    }

    #[staticmethod]
    fn surface_bright() -> PyDynamicColor {
        PyDynamicColor::material(Role::SurfaceBright)
    }

    #[staticmethod]
    fn surface_container_lowest() -> PyDynamicColor {
        PyDynamicColor::material(Role::SurfaceContainerLowest)
    }

    #[staticmethod]
    fn surface_container_low() -> PyDynamicColor {
        PyDynamicColor::material(Role::SurfaceContainerLow)
    }

    #[staticmethod]
    fn surface_container() -> PyDynamicColor {
        PyDynamicColor::material(Role::SurfaceContainer)
    }

    #[staticmethod]
    fn surface_container_high() -> PyDynamicColor {
        PyDynamicColor::material(Role::SurfaceContainerHigh)
    }

    #[staticmethod]
    fn surface_container_highest() -> PyDynamicColor {
        PyDynamicColor::material(Role::SurfaceContainerHighest)
    }

    #[staticmethod]
    fn on_surface() -> PyDynamicColor {
        PyDynamicColor::material(Role::OnSurface)
    }

    #[staticmethod]
    fn surface_variant() -> PyDynamicColor {
        PyDynamicColor::material(Role::SurfaceVariant)
    }

    #[staticmethod]
    fn on_surface_variant() -> PyDynamicColor {
        PyDynamicColor::material(Role::OnSurfaceVariant)
    }

    #[staticmethod]
    fn inverse_surface() -> PyDynamicColor {
        PyDynamicColor::material(Role::InverseSurface)
    }

    #[staticmethod]
    fn inverse_on_surface() -> PyDynamicColor {
        PyDynamicColor::material(Role::InverseOnSurface)
    }

    #[staticmethod]
    fn outline() -> PyDynamicColor {
        PyDynamicColor::material(Role::Outline)
    }

    #[staticmethod]
    fn outline_variant() -> PyDynamicColor {
        PyDynamicColor::material(Role::OutlineVariant)
    }

    #[staticmethod]
    fn shadow() -> PyDynamicColor {
        PyDynamicColor::material(Role::Shadow)
    }

    #[staticmethod]
    fn scrim() -> PyDynamicColor {
        PyDynamicColor::material(Role::Scrim)
    }

    #[staticmethod]
    fn surface_tint() -> PyDynamicColor {
        PyDynamicColor::material(Role::SurfaceTint)
    }

    #[staticmethod]
    fn primary() -> PyDynamicColor {
        PyDynamicColor::material(Role::Primary)
    }

    #[staticmethod]
    fn primary_dim() -> PyDynamicColor {
        PyDynamicColor::material(Role::PrimaryDim)
    }

    #[staticmethod]
    fn on_primary() -> PyDynamicColor {
        PyDynamicColor::material(Role::OnPrimary)
    }

    #[staticmethod]
    fn primary_container() -> PyDynamicColor {
        PyDynamicColor::material(Role::PrimaryContainer)
    }

    #[staticmethod]
    fn on_primary_container() -> PyDynamicColor {
        PyDynamicColor::material(Role::OnPrimaryContainer)
    }

    #[staticmethod]
    fn inverse_primary() -> PyDynamicColor {
        PyDynamicColor::material(Role::InversePrimary)
    }

    #[staticmethod]
    fn secondary() -> PyDynamicColor {
        PyDynamicColor::material(Role::Secondary)
    }

    #[staticmethod]
    fn secondary_dim() -> PyDynamicColor {
        PyDynamicColor::material(Role::SecondaryDim)
    }

    #[staticmethod]
    fn on_secondary() -> PyDynamicColor {
        PyDynamicColor::material(Role::OnSecondary)
    }

    #[staticmethod]
    fn secondary_container() -> PyDynamicColor {
        PyDynamicColor::material(Role::SecondaryContainer)
    }

    #[staticmethod]
    fn on_secondary_container() -> PyDynamicColor {
        PyDynamicColor::material(Role::OnSecondaryContainer)
    }

    #[staticmethod]
    fn tertiary() -> PyDynamicColor {
        PyDynamicColor::material(Role::Tertiary)
    }

    #[staticmethod]
    fn tertiary_dim() -> PyDynamicColor {
        PyDynamicColor::material(Role::TertiaryDim)
    }

    #[staticmethod]
    fn on_tertiary() -> PyDynamicColor {
        PyDynamicColor::material(Role::OnTertiary)
    }

    #[staticmethod]
    fn tertiary_container() -> PyDynamicColor {
        PyDynamicColor::material(Role::TertiaryContainer)
    }

    #[staticmethod]
    fn on_tertiary_container() -> PyDynamicColor {
        PyDynamicColor::material(Role::OnTertiaryContainer)
    }

    #[staticmethod]
    fn error() -> PyDynamicColor {
        PyDynamicColor::material(Role::Error)
    }

    #[staticmethod]
    fn error_dim() -> PyDynamicColor {
        PyDynamicColor::material(Role::ErrorDim)
    }

    #[staticmethod]
    fn on_error() -> PyDynamicColor {
        PyDynamicColor::material(Role::OnError)
    }

    #[staticmethod]
    fn error_container() -> PyDynamicColor {
        PyDynamicColor::material(Role::ErrorContainer)
    }

    #[staticmethod]
    fn on_error_container() -> PyDynamicColor {
        PyDynamicColor::material(Role::OnErrorContainer)
    }

    #[staticmethod]
    fn primary_fixed() -> PyDynamicColor {
        PyDynamicColor::material(Role::PrimaryFixed)
    }

    #[staticmethod]
    fn primary_fixed_dim() -> PyDynamicColor {
        PyDynamicColor::material(Role::PrimaryFixedDim)
    }

    #[staticmethod]
    fn on_primary_fixed() -> PyDynamicColor {
        PyDynamicColor::material(Role::OnPrimaryFixed)
    }

    #[staticmethod]
    fn on_primary_fixed_variant() -> PyDynamicColor {
        PyDynamicColor::material(Role::OnPrimaryFixedVariant)
    }

    #[staticmethod]
    fn secondary_fixed() -> PyDynamicColor {
        PyDynamicColor::material(Role::SecondaryFixed)
    }

    #[staticmethod]
    fn secondary_fixed_dim() -> PyDynamicColor {
        PyDynamicColor::material(Role::SecondaryFixedDim)
    }

    #[staticmethod]
    fn on_secondary_fixed() -> PyDynamicColor {
        PyDynamicColor::material(Role::OnSecondaryFixed)
    }

    #[staticmethod]
    fn on_secondary_fixed_variant() -> PyDynamicColor {
        PyDynamicColor::material(Role::OnSecondaryFixedVariant)
    }

    #[staticmethod]
    fn tertiary_fixed() -> PyDynamicColor {
        PyDynamicColor::material(Role::TertiaryFixed)
    }

    #[staticmethod]
    fn tertiary_fixed_dim() -> PyDynamicColor {
        PyDynamicColor::material(Role::TertiaryFixedDim)
    }

    #[staticmethod]
    fn on_tertiary_fixed() -> PyDynamicColor {
        PyDynamicColor::material(Role::OnTertiaryFixed)
    }

    #[staticmethod]
    fn on_tertiary_fixed_variant() -> PyDynamicColor {
        PyDynamicColor::material(Role::OnTertiaryFixedVariant)
    }
}
