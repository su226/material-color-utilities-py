use material_colors::hct::solver::HctSolver;
use pyo3::prelude::*;

use crate::{
    color::{PyRgb, rgb_to_py},
    hct::cam16::PyCam16,
};

#[pyfunction(name = "solve_to_rgb")]
pub fn py_solve_to_rgb(hue_degrees: f64, chroma: f64, lstar: f64) -> PyRgb {
    rgb_to_py(HctSolver::solve_to_rgb(hue_degrees, chroma, lstar))
}

#[pyfunction(name = "solve_to_cam")]
pub fn py_solve_to_cam(hue_degrees: f64, chroma: f64, lstar: f64) -> PyCam16 {
    PyCam16(HctSolver::solve_to_cam(hue_degrees, chroma, lstar))
}
