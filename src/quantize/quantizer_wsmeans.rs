use material_colors::quantize::QuantizerWsmeans;
use pyo3::prelude::*;

use crate::{
    color::PyRgb,
    pixels::{PyPixelFormat, pixels_from_bytes, pixels_from_list},
    quantize::quantizer::PyQuantizerResult,
};

#[pyfunction(name = "_quantize_wsmeans_rgb")]
pub fn py_quantize_wsmeans_rgb(
    input_pixels: Vec<PyRgb>,
    max_colors: usize,
    starting_clusters: Vec<PyRgb>,
) -> PyQuantizerResult {
    PyQuantizerResult(QuantizerWsmeans::quantize(
        &pixels_from_list(input_pixels),
        max_colors,
        &pixels_from_list(starting_clusters),
    ))
}

#[pyfunction(name = "_quantize_wsmeans_raw")]
#[pyo3(signature = (input_pixels, max_colors, starting_clusters, pixel_format = PyPixelFormat::Rgb, alpha_threshold = 128, premultiplied = false, width = None, stride = None))]
pub fn py_quantize_wsmeans_raw(
    input_pixels: &[u8],
    max_colors: usize,
    starting_clusters: Vec<PyRgb>,
    pixel_format: PyPixelFormat,
    alpha_threshold: u8,
    premultiplied: bool,
    width: Option<usize>,
    stride: Option<usize>,
) -> PyResult<PyQuantizerResult> {
    let input_pixels = pixels_from_bytes(
        input_pixels,
        pixel_format,
        alpha_threshold,
        premultiplied,
        width,
        stride,
    )?;
    let starting_clusters = pixels_from_list(starting_clusters);

    Ok(PyQuantizerResult(QuantizerWsmeans::quantize(
        &input_pixels,
        max_colors,
        &starting_clusters,
    )))
}
