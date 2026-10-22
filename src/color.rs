use std::str::FromStr;

use material_colors::color::{
    Lab, LinearRgb, Rgb, Xyz, delinearized, linearized, lstar_from_y, y_from_lstar,
};
use pyo3::{exceptions::PyValueError, prelude::*};

pub type PyRgb = (u8, u8, u8);
pub type PyLinearRgb = (f64, f64, f64);
pub type PyXyz = (f64, f64, f64);
pub type PyLab = (f64, f64, f64);

#[inline]
pub fn rgb_to_py(rgb: Rgb) -> PyRgb {
    (rgb.red, rgb.green, rgb.blue)
}

#[inline]
pub fn rgb_from_py(rgb: PyRgb) -> Rgb {
    Rgb::new(rgb.0, rgb.1, rgb.2)
}

#[inline]
pub fn linear_rgb_to_py(rgb: LinearRgb) -> PyLinearRgb {
    (rgb.red, rgb.green, rgb.blue)
}

#[inline]
pub fn linear_rgb_from_py(rgb: PyLinearRgb) -> LinearRgb {
    LinearRgb {
        red: rgb.0,
        green: rgb.1,
        blue: rgb.2,
    }
}

#[inline]
pub fn xyz_to_py(xyz: Xyz) -> PyXyz {
    (xyz.x, xyz.y, xyz.z)
}

#[inline]
pub fn xyz_from_py(xyz: PyXyz) -> Xyz {
    Xyz::new(xyz.0, xyz.1, xyz.2)
}

#[inline]
pub fn lab_to_py(lab: Lab) -> PyLab {
    (lab.l, lab.a, lab.b)
}

#[inline]
pub fn lab_from_py(lab: PyLab) -> Lab {
    Lab::new(lab.0, lab.1, lab.2)
}

#[pyfunction(name = "linear_rgb_to_rgb")]
pub fn py_linear_rgb_to_rgb(linear_rgb: PyLinearRgb) -> PyRgb {
    rgb_to_py(linear_rgb_from_py(linear_rgb).into())
}

#[pyfunction(name = "xyz_to_rgb")]
pub fn py_xyz_to_rgb(xyz: PyXyz) -> PyRgb {
    rgb_to_py(xyz_from_py(xyz).into())
}

#[pyfunction(name = "rgb_to_xyz")]
pub fn py_rgb_to_xyz(rgb: PyRgb) -> PyXyz {
    xyz_to_py(rgb_from_py(rgb).into())
}

#[pyfunction(name = "lab_to_rgb")]
pub fn py_lab_to_rgb(lab: PyLab) -> PyRgb {
    rgb_to_py(lab_from_py(lab).into())
}

#[pyfunction(name = "rgb_to_lab")]
pub fn py_rgb_to_lab(rgb: PyRgb) -> PyLab {
    lab_to_py(rgb_from_py(rgb).into())
}

#[pyfunction(name = "hex_to_rgb")]
pub fn py_hex_to_rgb(hex: &str) -> PyResult<PyRgb> {
    Rgb::from_str(hex)
        .map(rgb_to_py)
        .map_err(|x| PyValueError::new_err(x.to_string()))
}

#[pyfunction(name = "int_to_rgb")]
pub fn py_int_to_rgb(value: u32) -> PyRgb {
    rgb_to_py(Rgb::from_u32(value))
}

#[pyfunction(name = "lstar_to_rgb")]
pub fn py_lstar_to_rgb(lstar: f64) -> PyRgb {
    rgb_to_py(Rgb::from_lstar(lstar))
}

#[pyfunction(name = "rgb_to_lstar")]
pub fn py_rgb_to_lstar(rgb: PyRgb) -> f64 {
    rgb_from_py(rgb).as_lstar()
}

#[pyfunction(name = "rgb_to_int")]
pub fn py_rgb_to_int(rgb: PyRgb) -> u32 {
    rgb_from_py(rgb).as_u32()
}

#[pyfunction(name = "rgb_to_argb_int")]
pub fn py_rgb_to_argb_int(rgb: PyRgb) -> u32 {
    rgb_from_py(rgb).as_argb_u32()
}

#[pyfunction(name = "rgb_to_hex")]
pub fn py_rgb_to_hex(rgb: PyRgb) -> String {
    rgb_from_py(rgb).as_hex().to_string()
}

#[pyfunction(name = "y_from_lstar")]
pub fn py_y_from_lstar(lstar: f64) -> f64 {
    y_from_lstar(lstar)
}

#[pyfunction(name = "lstar_from_y")]
pub fn py_lstar_from_y(y: f64) -> f64 {
    lstar_from_y(y)
}

#[pyfunction(name = "linearized")]
pub fn py_linearized(rgb_component: u8) -> f64 {
    linearized(rgb_component)
}

#[pyfunction(name = "delinearized")]
pub fn py_delinearized(rgb_component: f64) -> u8 {
    delinearized(rgb_component)
}
