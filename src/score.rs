use indexmap::IndexMap;
use material_colors::score::Score;
use pyo3::prelude::*;

use crate::color::{PyRgb, rgb_from_py, rgb_to_py};

#[pyfunction(name = "score", signature = (colors_to_population, desired = None, fallback_color_rgb = None, filter = None))]
pub fn py_score(
    colors_to_population: IndexMap<PyRgb, u32>,
    desired: Option<usize>,
    fallback_color_rgb: Option<PyRgb>,
    filter: Option<bool>,
) -> Vec<PyRgb> {
    Score::score(
        &colors_to_population
            .into_iter()
            .map(|x| (rgb_from_py(x.0), x.1))
            .collect(),
        desired,
        fallback_color_rgb.map(rgb_from_py),
        filter,
    )
    .into_iter()
    .map(rgb_to_py)
    .collect()
}
