use material_colors::quantize::{Quantizer, QuantizerCelebi};
use pyo3::prelude::*;

use crate::{
    color::PyRgb,
    pixels::{PyPixelFormat, pixels_from_bytes, pixels_from_list},
    quantize::quantizer::PyQuantizerResult,
};

#[pyfunction(name = "_quantize_celebi_rgb")]
pub fn py_quantize_celebi_rgb(pixels: Vec<PyRgb>, max_colors: usize) -> PyQuantizerResult {
    PyQuantizerResult(QuantizerCelebi::quantize(
        &pixels_from_list(pixels),
        max_colors,
    ))
}

#[pyfunction(name = "_quantize_celebi_raw")]
#[pyo3(signature = (pixels, max_colors, pixel_format = PyPixelFormat::Rgb, alpha_threshold = 128, premultiplied = false, width = None, stride = None))]
pub fn py_quantize_celebi_raw(
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

    Ok(PyQuantizerResult(QuantizerCelebi::quantize(
        &pixels, max_colors,
    )))
}
