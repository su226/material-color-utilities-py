use material_colors::image::extract_color;
use pyo3::prelude::*;

use crate::{
    color::{PyRgb, rgb_to_py},
    pixels::{PyPixelFormat, pixels_from_bytes, pixels_from_list},
};

#[pyfunction(name = "_extract_color_raw")]
#[pyo3(signature = (image, pixel_format = PyPixelFormat::Rgb, alpha_threshold = 128, premultiplied = false, width = None, stride = None))]
pub fn py_extract_color_raw(
    image: &[u8],
    pixel_format: PyPixelFormat,
    alpha_threshold: u8,
    premultiplied: bool,
    width: Option<usize>,
    stride: Option<usize>,
) -> PyResult<PyRgb> {
    let pixels = pixels_from_bytes(
        image,
        pixel_format,
        alpha_threshold,
        premultiplied,
        width,
        stride,
    )?;

    Ok(rgb_to_py(extract_color(&pixels)))
}

#[pyfunction(name = "_extract_color_rgb")]
pub fn py_extract_color_rgb(pixels: Vec<PyRgb>) -> PyRgb {
    rgb_to_py(extract_color(&pixels_from_list(pixels)))
}
