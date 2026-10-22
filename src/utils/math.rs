use material_colors::utils::math::{
    difference_degrees, lerp, matrix_multiply, rotate_direction, sanitize_degrees_double,
    sanitize_degrees_int, signum,
};
use pyo3::prelude::*;

#[pyfunction(name = "signum")]
pub fn py_signum(value: f64) -> f64 {
    signum(value)
}

#[pyfunction(name = "lerp")]
pub fn py_lerp(start: f64, stop: f64, amount: f64) -> f64 {
    lerp(start, stop, amount)
}

#[pyfunction(name = "sanitize_degrees_int")]
pub fn py_sanitize_degrees_int(degrees: i32) -> u32 {
    sanitize_degrees_int(degrees)
}

#[pyfunction(name = "sanitize_degrees_double")]
pub fn py_sanitize_degrees_double(degrees: f64) -> f64 {
    sanitize_degrees_double(degrees)
}

#[pyfunction(name = "rotate_direction")]
pub fn py_rotate_direction(from_: f64, to: f64) -> f64 {
    rotate_direction(from_, to)
}

#[pyfunction(name = "difference_degrees")]
pub fn py_difference_degrees(a: f64, b: f64) -> f64 {
    difference_degrees(a, b)
}

#[pyfunction(name = "matrix_multiply")]
pub fn py_matrix_multiply(row: [f64; 3], matrix: [[f64; 3]; 3]) -> [f64; 3] {
    matrix_multiply(row, matrix)
}
