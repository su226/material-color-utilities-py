use material_colors::dynamic_color::Variant;
use pyo3::prelude::*;

#[pyclass(name = "Variant", from_py_object)]
#[derive(Clone)]
pub enum PyVariant {
    Monochrome,
    Neutral,
    TonalSpot,
    Vibrant,
    Expressive,
    Fidelity,
    Content,
    Rainbow,
    FruitSalad,
    Cmf,
}

impl From<PyVariant> for Variant {
    fn from(variant: PyVariant) -> Self {
        match variant {
            PyVariant::Monochrome => Self::Monochrome,
            PyVariant::Neutral => Self::Neutral,
            PyVariant::TonalSpot => Self::TonalSpot,
            PyVariant::Vibrant => Self::Vibrant,
            PyVariant::Expressive => Self::Expressive,
            PyVariant::Fidelity => Self::Fidelity,
            PyVariant::Content => Self::Content,
            PyVariant::Rainbow => Self::Rainbow,
            PyVariant::FruitSalad => Self::FruitSalad,
            PyVariant::Cmf => Self::Cmf,
        }
    }
}

impl From<Variant> for PyVariant {
    fn from(variant: Variant) -> Self {
        match variant {
            Variant::Monochrome => Self::Monochrome,
            Variant::Neutral => Self::Neutral,
            Variant::TonalSpot => Self::TonalSpot,
            Variant::Vibrant => Self::Vibrant,
            Variant::Expressive => Self::Expressive,
            Variant::Fidelity => Self::Fidelity,
            Variant::Content => Self::Content,
            Variant::Rainbow => Self::Rainbow,
            Variant::FruitSalad => Self::FruitSalad,
            Variant::Cmf => Self::Cmf,
        }
    }
}
