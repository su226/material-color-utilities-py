use material_colors::quantize::{PointProvider, PointProviderLab};
use pyo3::prelude::*;

use crate::color::{PyLab, PyRgb, lab_from_py, lab_to_py, rgb_from_py, rgb_to_py};

/// The standard CIE Lab point provider used by the quantizers.
#[pyclass(name = "PointProviderLab")]
#[derive(Clone, Copy)]
pub struct PyPointProviderLab;

#[pymethods]
impl PyPointProviderLab {
    /// Converts an RGB color to CIE Lab.
    #[staticmethod]
    fn lab_from_int(rgb: PyRgb) -> PyLab {
        lab_to_py(PointProviderLab::lab_from_int(&rgb_from_py(rgb)))
    }

    /// Converts a CIE Lab color to RGB.
    #[staticmethod]
    fn lab_to_int(lab: PyLab) -> PyRgb {
        rgb_to_py(PointProviderLab::lab_to_int(&lab_from_py(lab)))
    }

    /// Squared CIE Delta E 1976 distance between two Lab colors.
    #[staticmethod]
    fn distance(one: PyLab, two: PyLab) -> f64 {
        PointProviderLab::distance(&lab_from_py(one), &lab_from_py(two))
    }
}
