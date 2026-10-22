use material_colors::{
    dynamic_color::{Context, DynamicScheme, Platform, ToneCache},
    palette::Palette,
};
use pyo3::prelude::*;

use crate::{
    color::{PyRgb, rgb_from_py, rgb_to_py},
    dynamic_color::{PyDynamicColor, color_spec::PySpecVersion, variant::PyVariant},
    hct::PyHct,
    palette::{PyPalette, tonal::PyTonalPalette},
    scheme::PyScheme,
};

/// The platform on which a scheme is intended to be used.
#[pyclass(name = "Platform", from_py_object)]
#[derive(Clone, Copy)]
pub enum PyPlatform {
    Phone,
    Watch,
}

impl From<PyPlatform> for Platform {
    fn from(platform: PyPlatform) -> Self {
        match platform {
            PyPlatform::Phone => Self::Phone,
            PyPlatform::Watch => Self::Watch,
        }
    }
}

impl From<Platform> for PyPlatform {
    fn from(platform: Platform) -> Self {
        match platform {
            Platform::Phone => Self::Phone,
            Platform::Watch => Self::Watch,
        }
    }
}

/// A UI state for which colors can be resolved.
///
/// A scheme owns six tonal palettes and, together with a
/// [`DynamicColor`][crate::dynamic_color::PyDynamicColor], resolves colors for
/// the Material color roles.
#[pyclass(name = "DynamicScheme")]
#[derive(Clone)]
pub struct PyDynamicScheme(pub DynamicScheme);

#[pymethods]
impl PyDynamicScheme {
    #[new]
    #[pyo3(signature = (source_color_hct, variant, is_dark, contrast_level, primary_palette, secondary_palette, tertiary_palette, neutral_palette, neutral_variant_palette, error_palette = None))]
    pub fn new(
        source_color_hct: &PyHct,
        variant: PyVariant,
        is_dark: bool,
        contrast_level: Option<f64>,
        primary_palette: &PyTonalPalette,
        secondary_palette: &PyTonalPalette,
        tertiary_palette: &PyTonalPalette,
        neutral_palette: &PyTonalPalette,
        neutral_variant_palette: &PyTonalPalette,
        error_palette: Option<&PyTonalPalette>,
    ) -> Self {
        Self(DynamicScheme::new(
            source_color_hct.0,
            variant.into(),
            is_dark,
            contrast_level,
            primary_palette.0,
            secondary_palette.0,
            tertiary_palette.0,
            neutral_palette.0,
            neutral_variant_palette.0,
            error_palette.map(|palette| palette.0),
        ))
    }

    /// Builds a scheme whose palettes come from the given spec.
    #[staticmethod]
    #[pyo3(signature = (source_color_hct, variant, is_dark, contrast_level, platform, spec_version))]
    pub fn from_spec(
        source_color_hct: &PyHct,
        variant: PyVariant,
        is_dark: bool,
        contrast_level: Option<f64>,
        platform: PyPlatform,
        spec_version: PySpecVersion,
    ) -> Self {
        Self(DynamicScheme::from_spec(
            source_color_hct.0,
            variant.into(),
            is_dark,
            contrast_level,
            platform.into(),
            spec_version.into(),
        ))
    }

    /// Builds a scheme from an RGB source color using the variant's default
    /// palettes.
    #[staticmethod]
    #[pyo3(signature = (source, variant, is_dark, contrast_level = None))]
    pub fn by_variant(
        source: PyRgb,
        variant: PyVariant,
        is_dark: bool,
        contrast_level: Option<f64>,
    ) -> Self {
        Self(DynamicScheme::by_variant(
            rgb_from_py(source),
            &variant.into(),
            is_dark,
            contrast_level,
        ))
    }

    /// One palette of a scheme, built by the given spec.
    #[staticmethod]
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (variant, source_color_hct, palette, is_dark, platform, contrast_level, spec_version))]
    pub fn spec_palette(
        variant: PyVariant,
        source_color_hct: &PyHct,
        palette: PyPalette,
        is_dark: bool,
        platform: PyPlatform,
        contrast_level: f64,
        spec_version: PySpecVersion,
    ) -> PyTonalPalette {
        PyTonalPalette(DynamicScheme::spec_palette(
            variant.into(),
            source_color_hct.0,
            &palette.into(),
            is_dark,
            platform.into(),
            contrast_level,
            spec_version.into(),
        ))
    }

    /// Returns the spec version to use for `variant`, falling back when the
    /// variant is not supported by `spec_version`.
    #[staticmethod]
    pub fn fallback_spec_version(spec_version: PySpecVersion, variant: PyVariant) -> PySpecVersion {
        DynamicScheme::fallback_spec_version(spec_version.into(), variant.into()).into()
    }

    /// Piecewise mapping of `source_hue` through `hues`.
    #[staticmethod]
    #[pyo3(signature = (source_hue, hue_breakpoints, hues))]
    pub fn get_piecewise_value(source_hue: f64, hue_breakpoints: Vec<f64>, hues: Vec<f64>) -> f64 {
        DynamicScheme::get_piecewise_value(source_hue, &hue_breakpoints, &hues)
    }

    /// `source_hue` rotated by the value of its breakpoint segment.
    #[staticmethod]
    #[pyo3(signature = (source_hue, hue_breakpoints, rotations))]
    pub fn get_rotated_hue(source_hue: f64, hue_breakpoints: Vec<f64>, rotations: Vec<f64>) -> f64 {
        DynamicScheme::get_rotated_hue(source_hue, &hue_breakpoints, &rotations)
    }

    #[getter]
    fn get_source_color_hct(&self) -> PyHct {
        PyHct(self.0.source_color_hct)
    }

    #[setter]
    fn set_source_color_hct(&mut self, source_color_hct: &PyHct) {
        self.0.source_color_hct = source_color_hct.0;
    }

    #[getter]
    fn get_secondary_source_color_hct(&self) -> Option<PyHct> {
        self.0.secondary_source_color_hct.map(PyHct)
    }

    #[setter]
    fn set_secondary_source_color_hct(&mut self, hct: Option<&PyHct>) {
        self.0.secondary_source_color_hct = hct.map(|hct| hct.0);
    }

    #[getter]
    fn get_variant(&self) -> PyVariant {
        self.0.variant.into()
    }

    #[setter]
    fn set_variant(&mut self, variant: PyVariant) {
        self.0.variant = variant.into();
    }

    #[getter]
    fn get_is_dark(&self) -> bool {
        self.0.is_dark
    }

    #[setter]
    fn set_is_dark(&mut self, is_dark: bool) {
        self.0.is_dark = is_dark;
    }

    #[getter]
    fn get_contrast_level(&self) -> f64 {
        self.0.contrast_level
    }

    #[setter]
    fn set_contrast_level(&mut self, contrast_level: f64) {
        self.0.contrast_level = contrast_level;
    }

    #[getter]
    fn get_platform(&self) -> PyPlatform {
        self.0.platform.into()
    }

    #[setter]
    fn set_platform(&mut self, platform: PyPlatform) {
        self.0.platform = platform.into();
    }

    #[getter]
    fn get_spec_version(&self) -> PySpecVersion {
        self.0.spec_version.into()
    }

    #[setter]
    fn set_spec_version(&mut self, spec_version: PySpecVersion) {
        self.0.spec_version =
            DynamicScheme::fallback_spec_version(spec_version.into(), self.0.variant);
    }

    #[getter]
    fn get_primary_palette(&self) -> PyTonalPalette {
        PyTonalPalette(self.0.primary_palette)
    }

    #[setter]
    fn set_primary_palette(&mut self, palette: &PyTonalPalette) {
        self.0.primary_palette = palette.0;
    }

    #[getter]
    fn get_secondary_palette(&self) -> PyTonalPalette {
        PyTonalPalette(self.0.secondary_palette)
    }

    #[setter]
    fn set_secondary_palette(&mut self, palette: &PyTonalPalette) {
        self.0.secondary_palette = palette.0;
    }

    #[getter]
    fn get_tertiary_palette(&self) -> PyTonalPalette {
        PyTonalPalette(self.0.tertiary_palette)
    }

    #[setter]
    fn set_tertiary_palette(&mut self, palette: &PyTonalPalette) {
        self.0.tertiary_palette = palette.0;
    }

    #[getter]
    fn get_neutral_palette(&self) -> PyTonalPalette {
        PyTonalPalette(self.0.neutral_palette)
    }

    #[setter]
    fn set_neutral_palette(&mut self, palette: &PyTonalPalette) {
        self.0.neutral_palette = palette.0;
    }

    #[getter]
    fn get_neutral_variant_palette(&self) -> PyTonalPalette {
        PyTonalPalette(self.0.neutral_variant_palette)
    }

    #[setter]
    fn set_neutral_variant_palette(&mut self, palette: &PyTonalPalette) {
        self.0.neutral_variant_palette = palette.0;
    }

    #[getter]
    fn get_error_palette(&self) -> PyTonalPalette {
        PyTonalPalette(self.0.error_palette)
    }

    #[setter]
    fn set_error_palette(&mut self, palette: &PyTonalPalette) {
        self.0.error_palette = palette.0;
    }

    /// Sets the spec version, applying the variant fallback.
    fn with_spec_version(&self, spec_version: PySpecVersion) -> Self {
        Self(self.0.clone().with_spec_version(spec_version.into()))
    }

    /// Returns a copy of this scheme with another platform.
    fn with_platform(&self, platform: PyPlatform) -> Self {
        Self(self.0.clone().with_platform(platform.into()))
    }

    /// Returns a copy of this scheme with another optional second source
    /// color.
    fn with_secondary_source_color_hct(&self, hct: Option<&PyHct>) -> Self {
        Self(
            self.0
                .clone()
                .with_secondary_source_color_hct(hct.map(|hct| hct.0)),
        )
    }

    /// Returns a copy of this scheme with another mode and contrast level.
    fn with_mode(&self, is_dark: bool, contrast_level: f64) -> Self {
        Self(self.0.with_mode(is_dark, contrast_level))
    }

    /// One of the six palettes owned by this scheme.
    fn palette(&self, palette: PyPalette) -> PyTonalPalette {
        let palette: Palette = palette.into();
        let palette = match palette {
            Palette::Primary => material_colors::dynamic_color::SchemePalette::Primary,
            Palette::Secondary => material_colors::dynamic_color::SchemePalette::Secondary,
            Palette::Tertiary => material_colors::dynamic_color::SchemePalette::Tertiary,
            Palette::Neutral => material_colors::dynamic_color::SchemePalette::Neutral,
            Palette::NeutralVariant => {
                material_colors::dynamic_color::SchemePalette::NeutralVariant
            }
            Palette::Error => material_colors::dynamic_color::SchemePalette::Error,
        };

        PyTonalPalette(*self.0.palette(palette))
    }

    /// The second source color, or the first if there is none.
    fn secondary_source_or_primary(&self) -> PyHct {
        PyHct(self.0.secondary_source_or_primary())
    }

    /// A resolver sharing one tone memo. Use it when reading many colors.
    fn resolver(&self) -> PySchemeResolver {
        PySchemeResolver {
            scheme: self.0.clone(),
            cache: ToneCache::new(),
        }
    }

    /// Resolves the HCT color of `color` for this scheme.
    fn get_hct(&self, color: &PyDynamicColor) -> PyHct {
        PyHct(self.0.get_hct(color.as_dynamic()))
    }

    /// Resolves the RGB color of `color` for this scheme.
    fn get_rgb(&self, color: &PyDynamicColor) -> PyRgb {
        rgb_to_py(self.0.get_rgb(color.as_dynamic()))
    }

    /// Converts this dynamic scheme into a plain 49-color [`Scheme`].
    fn to_scheme(&self) -> PyScheme {
        PyScheme(material_colors::scheme::Scheme::from(self.0.clone()))
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __repr__(&self) -> String {
        format!(
            "DynamicScheme(variant={:?}, is_dark={}, spec_version={:?})",
            self.0.variant, self.0.is_dark, self.0.spec_version
        )
    }

    /// Resolves this Material color role to RGB.
    fn primary_palette_key_color(&self) -> PyRgb {
        rgb_to_py(self.0.primary_palette_key_color())
    }

    /// Resolves this Material color role to RGB.
    fn secondary_palette_key_color(&self) -> PyRgb {
        rgb_to_py(self.0.secondary_palette_key_color())
    }

    /// Resolves this Material color role to RGB.
    fn tertiary_palette_key_color(&self) -> PyRgb {
        rgb_to_py(self.0.tertiary_palette_key_color())
    }

    /// Resolves this Material color role to RGB.
    fn neutral_palette_key_color(&self) -> PyRgb {
        rgb_to_py(self.0.neutral_palette_key_color())
    }

    /// Resolves this Material color role to RGB.
    fn neutral_variant_palette_key_color(&self) -> PyRgb {
        rgb_to_py(self.0.neutral_variant_palette_key_color())
    }

    /// Resolves this Material color role to RGB.
    fn error_palette_key_color(&self) -> PyRgb {
        rgb_to_py(self.0.error_palette_key_color())
    }

    /// Resolves this Material color role to RGB.
    fn background(&self) -> PyRgb {
        rgb_to_py(self.0.background())
    }

    /// Resolves this Material color role to RGB.
    fn on_background(&self) -> PyRgb {
        rgb_to_py(self.0.on_background())
    }

    /// Resolves this Material color role to RGB.
    fn surface(&self) -> PyRgb {
        rgb_to_py(self.0.surface())
    }

    /// Resolves this Material color role to RGB.
    fn surface_dim(&self) -> PyRgb {
        rgb_to_py(self.0.surface_dim())
    }

    /// Resolves this Material color role to RGB.
    fn surface_bright(&self) -> PyRgb {
        rgb_to_py(self.0.surface_bright())
    }

    /// Resolves this Material color role to RGB.
    fn surface_container_lowest(&self) -> PyRgb {
        rgb_to_py(self.0.surface_container_lowest())
    }

    /// Resolves this Material color role to RGB.
    fn surface_container_low(&self) -> PyRgb {
        rgb_to_py(self.0.surface_container_low())
    }

    /// Resolves this Material color role to RGB.
    fn surface_container(&self) -> PyRgb {
        rgb_to_py(self.0.surface_container())
    }

    /// Resolves this Material color role to RGB.
    fn surface_container_high(&self) -> PyRgb {
        rgb_to_py(self.0.surface_container_high())
    }

    /// Resolves this Material color role to RGB.
    fn surface_container_highest(&self) -> PyRgb {
        rgb_to_py(self.0.surface_container_highest())
    }

    /// Resolves this Material color role to RGB.
    fn on_surface(&self) -> PyRgb {
        rgb_to_py(self.0.on_surface())
    }

    /// Resolves this Material color role to RGB.
    fn surface_variant(&self) -> PyRgb {
        rgb_to_py(self.0.surface_variant())
    }

    /// Resolves this Material color role to RGB.
    fn on_surface_variant(&self) -> PyRgb {
        rgb_to_py(self.0.on_surface_variant())
    }

    /// Resolves this Material color role to RGB.
    fn inverse_surface(&self) -> PyRgb {
        rgb_to_py(self.0.inverse_surface())
    }

    /// Resolves this Material color role to RGB.
    fn inverse_on_surface(&self) -> PyRgb {
        rgb_to_py(self.0.inverse_on_surface())
    }

    /// Resolves this Material color role to RGB.
    fn outline(&self) -> PyRgb {
        rgb_to_py(self.0.outline())
    }

    /// Resolves this Material color role to RGB.
    fn outline_variant(&self) -> PyRgb {
        rgb_to_py(self.0.outline_variant())
    }

    /// Resolves this Material color role to RGB.
    fn shadow(&self) -> PyRgb {
        rgb_to_py(self.0.shadow())
    }

    /// Resolves this Material color role to RGB.
    fn scrim(&self) -> PyRgb {
        rgb_to_py(self.0.scrim())
    }

    /// Resolves this Material color role to RGB.
    fn surface_tint(&self) -> PyRgb {
        rgb_to_py(self.0.surface_tint())
    }

    /// Resolves this Material color role to RGB.
    fn primary(&self) -> PyRgb {
        rgb_to_py(self.0.primary())
    }

    /// Resolves this Material color role to RGB.
    fn primary_dim(&self) -> PyRgb {
        rgb_to_py(self.0.primary_dim())
    }

    /// Resolves this Material color role to RGB.
    fn on_primary(&self) -> PyRgb {
        rgb_to_py(self.0.on_primary())
    }

    /// Resolves this Material color role to RGB.
    fn primary_container(&self) -> PyRgb {
        rgb_to_py(self.0.primary_container())
    }

    /// Resolves this Material color role to RGB.
    fn on_primary_container(&self) -> PyRgb {
        rgb_to_py(self.0.on_primary_container())
    }

    /// Resolves this Material color role to RGB.
    fn inverse_primary(&self) -> PyRgb {
        rgb_to_py(self.0.inverse_primary())
    }

    /// Resolves this Material color role to RGB.
    fn secondary(&self) -> PyRgb {
        rgb_to_py(self.0.secondary())
    }

    /// Resolves this Material color role to RGB.
    fn secondary_dim(&self) -> PyRgb {
        rgb_to_py(self.0.secondary_dim())
    }

    /// Resolves this Material color role to RGB.
    fn on_secondary(&self) -> PyRgb {
        rgb_to_py(self.0.on_secondary())
    }

    /// Resolves this Material color role to RGB.
    fn secondary_container(&self) -> PyRgb {
        rgb_to_py(self.0.secondary_container())
    }

    /// Resolves this Material color role to RGB.
    fn on_secondary_container(&self) -> PyRgb {
        rgb_to_py(self.0.on_secondary_container())
    }

    /// Resolves this Material color role to RGB.
    fn tertiary(&self) -> PyRgb {
        rgb_to_py(self.0.tertiary())
    }

    /// Resolves this Material color role to RGB.
    fn tertiary_dim(&self) -> PyRgb {
        rgb_to_py(self.0.tertiary_dim())
    }

    /// Resolves this Material color role to RGB.
    fn on_tertiary(&self) -> PyRgb {
        rgb_to_py(self.0.on_tertiary())
    }

    /// Resolves this Material color role to RGB.
    fn tertiary_container(&self) -> PyRgb {
        rgb_to_py(self.0.tertiary_container())
    }

    /// Resolves this Material color role to RGB.
    fn on_tertiary_container(&self) -> PyRgb {
        rgb_to_py(self.0.on_tertiary_container())
    }

    /// Resolves this Material color role to RGB.
    fn error(&self) -> PyRgb {
        rgb_to_py(self.0.error())
    }

    /// Resolves this Material color role to RGB.
    fn error_dim(&self) -> PyRgb {
        rgb_to_py(self.0.error_dim())
    }

    /// Resolves this Material color role to RGB.
    fn on_error(&self) -> PyRgb {
        rgb_to_py(self.0.on_error())
    }

    /// Resolves this Material color role to RGB.
    fn error_container(&self) -> PyRgb {
        rgb_to_py(self.0.error_container())
    }

    /// Resolves this Material color role to RGB.
    fn on_error_container(&self) -> PyRgb {
        rgb_to_py(self.0.on_error_container())
    }

    /// Resolves this Material color role to RGB.
    fn primary_fixed(&self) -> PyRgb {
        rgb_to_py(self.0.primary_fixed())
    }

    /// Resolves this Material color role to RGB.
    fn primary_fixed_dim(&self) -> PyRgb {
        rgb_to_py(self.0.primary_fixed_dim())
    }

    /// Resolves this Material color role to RGB.
    fn on_primary_fixed(&self) -> PyRgb {
        rgb_to_py(self.0.on_primary_fixed())
    }

    /// Resolves this Material color role to RGB.
    fn on_primary_fixed_variant(&self) -> PyRgb {
        rgb_to_py(self.0.on_primary_fixed_variant())
    }

    /// Resolves this Material color role to RGB.
    fn secondary_fixed(&self) -> PyRgb {
        rgb_to_py(self.0.secondary_fixed())
    }

    /// Resolves this Material color role to RGB.
    fn secondary_fixed_dim(&self) -> PyRgb {
        rgb_to_py(self.0.secondary_fixed_dim())
    }

    /// Resolves this Material color role to RGB.
    fn on_secondary_fixed(&self) -> PyRgb {
        rgb_to_py(self.0.on_secondary_fixed())
    }

    /// Resolves this Material color role to RGB.
    fn on_secondary_fixed_variant(&self) -> PyRgb {
        rgb_to_py(self.0.on_secondary_fixed_variant())
    }

    /// Resolves this Material color role to RGB.
    fn tertiary_fixed(&self) -> PyRgb {
        rgb_to_py(self.0.tertiary_fixed())
    }

    /// Resolves this Material color role to RGB.
    fn tertiary_fixed_dim(&self) -> PyRgb {
        rgb_to_py(self.0.tertiary_fixed_dim())
    }

    /// Resolves this Material color role to RGB.
    fn on_tertiary_fixed(&self) -> PyRgb {
        rgb_to_py(self.0.on_tertiary_fixed())
    }

    /// Resolves this Material color role to RGB.
    fn on_tertiary_fixed_variant(&self) -> PyRgb {
        rgb_to_py(self.0.on_tertiary_fixed_variant())
    }
}

/// Resolves many colors of one scheme while sharing the tone memo.
#[pyclass(name = "SchemeResolver", unsendable)]
pub struct PySchemeResolver {
    scheme: DynamicScheme,
    cache: ToneCache,
}

#[pymethods]
impl PySchemeResolver {
    /// The resolved tone of `color`.
    fn tone(&self, color: &PyDynamicColor) -> f64 {
        Context::new(&self.scheme, &self.cache).tone(color.as_dynamic())
    }

    /// The resolved HCT color of `color`.
    fn hct(&self, color: &PyDynamicColor) -> PyHct {
        PyHct(Context::new(&self.scheme, &self.cache).hct(color.as_dynamic()))
    }

    /// The resolved RGB color of `color`.
    fn rgb(&self, color: &PyDynamicColor) -> PyRgb {
        let hct = Context::new(&self.scheme, &self.cache).hct(color.as_dynamic());

        rgb_to_py(hct.into())
    }

    #[getter]
    fn get_scheme(&self) -> PyDynamicScheme {
        PyDynamicScheme(self.scheme.clone())
    }
}
