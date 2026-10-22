use material_colors::palette::Palette;
use pyo3::prelude::*;

pub mod core;
pub mod tonal;

#[pyclass(name = "Palette", from_py_object)]
#[derive(Clone)]
pub enum PyPalette {
    Primary,
    Secondary,
    Tertiary,
    Error,
    Neutral,
    NeutralVariant,
}

impl From<PyPalette> for Palette {
    fn from(palette: PyPalette) -> Self {
        match palette {
            PyPalette::Primary => Self::Primary,
            PyPalette::Secondary => Self::Secondary,
            PyPalette::Tertiary => Self::Tertiary,
            PyPalette::Error => Self::Error,
            PyPalette::Neutral => Self::Neutral,
            PyPalette::NeutralVariant => Self::NeutralVariant,
        }
    }
}

impl From<Palette> for PyPalette {
    fn from(palette: Palette) -> Self {
        match palette {
            Palette::Primary => Self::Primary,
            Palette::Secondary => Self::Secondary,
            Palette::Tertiary => Self::Tertiary,
            Palette::Error => Self::Error,
            Palette::Neutral => Self::Neutral,
            Palette::NeutralVariant => Self::NeutralVariant,
        }
    }
}
