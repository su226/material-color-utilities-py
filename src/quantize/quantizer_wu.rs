use material_colors::quantize::{Quantizer, QuantizerWu};
use pyo3::prelude::*;

use crate::{
    color::PyRgb,
    pixels::{PyPixelFormat, pixels_from_bytes, pixels_from_list},
    quantize::quantizer::PyQuantizerResult,
};

#[pyfunction(name = "_quantize_wu_rgb")]
pub fn py_quantize_wu_rgb(pixels: Vec<PyRgb>, max_colors: usize) -> PyQuantizerResult {
    PyQuantizerResult(QuantizerWu::quantize(&pixels_from_list(pixels), max_colors))
}

#[pyfunction(name = "_quantize_wu_raw")]
#[pyo3(signature = (pixels, max_colors, pixel_format = PyPixelFormat::Rgb, alpha_threshold = 128, premultiplied = false, width = None, stride = None))]
pub fn py_quantize_wu_raw(
    pixels: &[u8],
    max_colors: usize,
    pixel_format: PyPixelFormat,
    alpha_threshold: u8,
    premultiplied: bool,
    width: Option<usize>,
    stride: Option<usize>,
) -> PyResult<PyQuantizerResult> {
    let pixels = pixels_from_bytes(
        pixels,
        pixel_format,
        alpha_threshold,
        premultiplied,
        width,
        stride,
    )?;

    Ok(PyQuantizerResult(QuantizerWu::quantize(
        &pixels, max_colors,
    )))
}
