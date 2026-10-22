//! Bindings for the dynamic color system.
//!
//! A [`PyDynamicColor`] is a color role that adjusts itself based on UI state,
//! represented by a [`PyDynamicScheme`].

pub mod color_spec;
pub mod color_spec_2021;
pub mod color_spec_2025;
pub mod color_spec_2026;
pub mod contrast_curve;
pub mod dynamic_scheme;
pub mod material_dynamic_colors;
pub mod tone_delta_pair;
pub mod variant;

use material_colors::{
    dynamic_color::{CustomColor, DynamicColor, DynamicScheme, Role},
    palette::TonalPalette,
};
use pyo3::{exceptions::PyValueError, prelude::*};

use crate::{
    color::{PyRgb, rgb_to_py},
    dynamic_color::{
        contrast_curve::PyContrastCurve, dynamic_scheme::PyDynamicScheme,
        tone_delta_pair::PyToneDeltaPair,
    },
    hct::PyHct,
    palette::tonal::PyTonalPalette,
};

/// A color with a fixed hue/chroma/tone and no background.
///
/// This is the Python counterpart of `material_colors::dynamic_color::ArgbColor`.
/// The Rust type stores its name as `&'static str`; this wrapper owns a
/// `String`, so names can come from Python without leaking memory.
#[pyclass(name = "ArgbColor")]
#[derive(Clone)]
pub struct PyArgbColor {
    pub name: String,
    pub palette: TonalPalette,
    pub tone: f64,
}

impl CustomColor for PyArgbColor {
    fn name(&self) -> &str {
        &self.name
    }

    fn palette<'s>(&'s self, _: &'s DynamicScheme) -> &'s TonalPalette {
        &self.palette
    }

    fn tone(&self, _: &DynamicScheme) -> Option<f64> {
        Some(self.tone)
    }
}

#[pymethods]
impl PyArgbColor {
    /// Creates a fixed color from a name and an HCT color.
    #[new]
    pub fn new(name: &str, hct: &PyHct) -> Self {
        Self {
            name: name.to_owned(),
            palette: TonalPalette::from_hct(hct.0),
            tone: hct.0.get_tone(),
        }
    }

    #[getter]
    fn get_name(&self) -> String {
        self.name.clone()
    }

    #[setter]
    fn set_name(&mut self, name: &str) {
        self.name = name.to_owned();
    }

    /// The tonal palette generated from the source color.
    #[getter]
    fn get_palette(&self) -> PyTonalPalette {
        PyTonalPalette(self.palette)
    }

    #[setter]
    fn set_palette(&mut self, palette: &PyTonalPalette) {
        self.palette = palette.0;
    }

    /// The fixed tone this color resolves to.
    #[getter]
    fn get_tone(&self) -> f64 {
        self.tone
    }

    #[setter]
    fn set_tone(&mut self, tone: f64) {
        self.tone = tone;
    }

    /// Wraps this fixed color in a [`DynamicColor`].
    ///
    /// The resulting dynamic color owns a snapshot of this value; changing
    /// this `ArgbColor` afterwards does not affect it.
    fn to_dynamic_color(&self) -> PyDynamicColor {
        PyDynamicColor(PyDynamicColorKind::Custom(self.clone()))
    }

    fn __str__(&self) -> String {
        self.name.clone()
    }

    fn __repr__(&self) -> String {
        format!("ArgbColor({:?}, tone={})", self.name, self.tone)
    }
}

/// The concrete backing data of a [`PyDynamicColor`].
#[derive(Clone)]
pub enum PyDynamicColorKind {
    /// One of the built-in Material color roles.
    Material(Role),
    /// A user-defined fixed color.
    Custom(PyArgbColor),
}

/// A color role that resolves to a tone, HCT color or RGB color for a given
/// [`PyDynamicScheme`].
///
/// A dynamic color is either one of the built-in Material roles or an
/// [`ArgbColor`]. Use
/// [`MaterialDynamicColors`][crate::dynamic_color::material_dynamic_colors::PyMaterialDynamicColors]
/// or [`ArgbColor.to_dynamic_color`] to obtain one.
#[pyclass(name = "DynamicColor")]
#[derive(Clone)]
pub struct PyDynamicColor(pub(crate) PyDynamicColorKind);

impl PyDynamicColor {
    /// Wraps a built-in Material color role.
    pub fn material(role: Role) -> Self {
        Self(PyDynamicColorKind::Material(role))
    }

    /// Wraps a Rust dynamic color.
    ///
    /// Returns `None` for user-defined colors, whose concrete Rust type cannot
    /// be recovered from a trait object. Use [`Self::from_argb`] for
    /// [`PyArgbColor`] values.
    pub fn from_dynamic(color: DynamicColor<'_>) -> Option<Self> {
        match color {
            DynamicColor::Material(role) => Some(Self::material(role)),
            DynamicColor::Custom(_) => None,
        }
    }

    /// The token name of the color.
    pub fn name(&self) -> &str {
        match &self.0 {
            PyDynamicColorKind::Material(role) => role.name(),
            PyDynamicColorKind::Custom(color) => &color.name,
        }
    }

    /// Borrows the underlying Rust dynamic color.
    pub fn as_dynamic(&self) -> DynamicColor<'_> {
        match &self.0 {
            PyDynamicColorKind::Material(role) => DynamicColor::Material(*role),
            PyDynamicColorKind::Custom(color) => DynamicColor::Custom(color),
        }
    }
}

#[pymethods]
impl PyDynamicColor {
    /// Creates a built-in color role from its token name, e.g. `"primary"`
    /// or `"on_primary_container"`.
    #[new]
    pub fn new(name: &str) -> PyResult<Self> {
        Role::ALL
            .iter()
            .copied()
            .find(|role| role.name() == name)
            .map(Self::material)
            .ok_or_else(|| PyValueError::new_err(format!("Unknown color role: {name}")))
    }

    /// Creates a dynamic color from a fixed [`ArgbColor`].
    ///
    /// The dynamic color owns a snapshot of `color`; later changes to
    /// `color` do not affect it.
    #[staticmethod]
    pub fn from_argb(color: &PyArgbColor) -> Self {
        Self(PyDynamicColorKind::Custom(color.clone()))
    }

    /// The token name, e.g. `"on_primary_container"`.
    #[getter]
    fn get_name(&self) -> String {
        self.name().to_owned()
    }

    /// Returns the tone in HCT, ranging from 0 to 100, of the resolved color.
    fn get_tone(&self, scheme: &PyDynamicScheme) -> f64 {
        self.as_dynamic().get_tone(&scheme.0)
    }

    /// Returns the resolved color in HCT.
    fn get_hct(&self, scheme: &PyDynamicScheme) -> PyHct {
        PyHct(self.as_dynamic().get_hct(&scheme.0))
    }

    /// Returns the resolved color in RGB.
    fn get_rgb(&self, scheme: &PyDynamicScheme) -> PyRgb {
        rgb_to_py(self.as_dynamic().get_rgb(&scheme.0))
    }

    /// Alpha channel from `opacity`, 255 when unset.
    fn get_alpha(&self, scheme: &PyDynamicScheme) -> u8 {
        self.as_dynamic().get_alpha(&scheme.0)
    }

    /// The tonal palette this color is resolved from.
    fn palette(&self, scheme: &PyDynamicScheme) -> PyTonalPalette {
        let color = self.as_dynamic();

        PyTonalPalette(*color.palette(&scheme.0))
    }

    /// Whether this color is a background color.
    fn is_background(&self, scheme: &PyDynamicScheme) -> bool {
        self.as_dynamic().is_background(&scheme.0)
    }

    /// The background color this color contrasts with, if any.
    fn background(&self, scheme: &PyDynamicScheme) -> Option<PyDynamicColor> {
        self.as_dynamic()
            .background(&scheme.0)
            .and_then(Self::from_dynamic)
    }

    /// An optional second background color.
    fn second_background(&self, scheme: &PyDynamicScheme) -> Option<PyDynamicColor> {
        self.as_dynamic()
            .second_background(&scheme.0)
            .and_then(Self::from_dynamic)
    }

    /// The contrast curve of this color, if it has one.
    fn contrast_curve(&self, scheme: &PyDynamicScheme) -> Option<PyContrastCurve> {
        self.as_dynamic()
            .contrast_curve(&scheme.0)
            .map(PyContrastCurve)
    }

    /// The tone delta pair constraint of this color, if it has one.
    fn tone_delta_pair(&self, scheme: &PyDynamicScheme) -> Option<PyToneDeltaPair> {
        self.as_dynamic()
            .tone_delta_pair(&scheme.0)
            .and_then(PyToneDeltaPair::from_rust)
    }

    /// The chroma multiplier of this color, if it has one.
    fn chroma_multiplier(&self, scheme: &PyDynamicScheme) -> Option<f64> {
        self.as_dynamic().chroma_multiplier(&scheme.0)
    }

    /// Whether `other` refers to the same color role or custom color name.
    fn same_as(&self, other: &Self) -> bool {
        self.name() == other.name()
    }

    /// Given a background tone, find a foreground tone, while ensuring they
    /// reach a contrast ratio that is as close to `ratio` as possible.
    #[staticmethod]
    fn foreground_tone(bg_tone: f64, ratio: f64) -> f64 {
        DynamicColor::foreground_tone(bg_tone, ratio)
    }

    /// Adjust a tone down such that white has 4.5 contrast, if the tone is
    /// reasonably close to supporting it.
    #[staticmethod]
    fn enable_light_foreground(tone: f64) -> f64 {
        DynamicColor::enable_light_foreground(tone)
    }

    /// People prefer white foregrounds on ~T60-70.
    #[staticmethod]
    fn tone_prefers_light_foreground(tone: f64) -> bool {
        DynamicColor::tone_prefers_light_foreground(tone)
    }

    /// Tones less than ~T50 always permit white at 4.5 contrast.
    #[staticmethod]
    fn tone_allows_light_foreground(tone: f64) -> bool {
        DynamicColor::tone_allows_light_foreground(tone)
    }

    fn __str__(&self) -> String {
        self.name().to_owned()
    }

    fn __repr__(&self) -> String {
        format!("DynamicColor({:?})", self.name())
    }
}
