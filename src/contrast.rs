use material_colors::contrast::{darker, darker_unsafe, lighter, lighter_unsafe, ratio_of_tones};
use pyo3::prelude::*;

#[pyfunction(name = "ratio_of_tones")]
pub fn py_ratio_of_tones(tone_a: f64, tone_b: f64) -> f64 {
    ratio_of_tones(tone_a, tone_b)
}

#[pyfunction(name = "lighter")]
pub fn py_lighter(tone: f64, ratio: f64) -> Option<f64> {
    lighter(tone, ratio)
}

#[pyfunction(name = "darker")]
pub fn py_darker(tone: f64, ratio: f64) -> Option<f64> {
    darker(tone, ratio)
}

#[pyfunction(name = "lighter_unsafe")]
pub fn py_lighter_unsafe(tone: f64, ratio: f64) -> f64 {
    lighter_unsafe(tone, ratio)
}

#[pyfunction(name = "darker_unsafe")]
pub fn py_darker_unsafe(tone: f64, ratio: f64) -> f64 {
    darker_unsafe(tone, ratio)
}
