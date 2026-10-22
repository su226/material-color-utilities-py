use material_colors::dynamic_color::SpecVersion;
use pyo3::prelude::*;

#[pyclass(name = "SpecVersion", from_py_object)]
#[derive(Clone)]
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
