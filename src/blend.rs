use material_colors::blend::{cam16_ucs, harmonize, hct_hue};
use pyo3::pyfunction;

use crate::color::{PyRgb, rgb_from_py, rgb_to_py};

#[pyfunction(name = "harmonize")]
pub fn py_harmonize(design_color: PyRgb, source_color: PyRgb) -> PyRgb {
    rgb_to_py(harmonize(
        rgb_from_py(design_color),
        rgb_from_py(source_color),
    ))
}

#[pyfunction(name = "hct_hue")]
pub fn py_hct_hue(from_: PyRgb, to: PyRgb, amount: f64) -> PyRgb {
    rgb_to_py(hct_hue(rgb_from_py(from_), rgb_from_py(to), amount))
}

#[pyfunction(name = "cam16_ucs")]
pub fn py_cam16_ucs(from_: PyRgb, to: PyRgb, amount: f64) -> PyRgb {
    rgb_to_py(cam16_ucs(rgb_from_py(from_), rgb_from_py(to), amount))
}
