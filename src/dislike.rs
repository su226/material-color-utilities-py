use material_colors::dislike::{fix_if_disliked, is_disliked};
use pyo3::prelude::*;

use crate::hct::PyHct;

#[pyfunction(name = "is_disliked")]
pub fn py_is_disliked(hct: &PyHct) -> bool {
    is_disliked(&hct.0)
}

#[pyfunction(name = "fix_if_disliked")]
pub fn py_fix_if_disliked(hct: &PyHct) -> PyHct {
    PyHct(fix_if_disliked(hct.0))
}
