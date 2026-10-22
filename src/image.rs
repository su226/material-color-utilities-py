use arrow_array::{Array, FixedSizeListArray, UInt8Array};
use material_colors::{color::Rgb, image::extract_color};
use pyo3::{exceptions::PyTypeError, prelude::*};
use pyo3_arrow::PyArray;

use crate::color::{PyRgb, rgb_to_py};

#[pyfunction(name = "_extract_color_raw")]
pub fn py_extract_color_raw(image: &[u8]) -> PyRgb {
    let mut pixels = Vec::with_capacity(image.len() / 3);
    pixels.extend(
        image
            .as_chunks::<3>()
            .0
            .iter()
            .map(|x| Rgb::new(x[0], x[1], x[2])),
    );
    rgb_to_py(extract_color(&pixels))
}

#[pyfunction(name = "_extract_color_pil")]
pub fn py_extract_color_pil(image: PyArray, alpha_threshold: u8) -> PyResult<PyRgb> {
    let array = image
        .array()
        .as_any()
        .downcast_ref::<FixedSizeListArray>()
        .ok_or_else(|| PyTypeError::new_err("Invalid image"))?;
    let values = array
        .values()
        .as_any()
        .downcast_ref::<UInt8Array>()
        .ok_or_else(|| PyTypeError::new_err("Invalid image"))?;
    let mut pixels = Vec::with_capacity(array.len());
    if alpha_threshold == 0 {
        pixels.extend(
            values
                .values()
                .as_chunks::<4>()
                .0
                .iter()
                .map(|x| Rgb::new(x[0], x[1], x[2])),
        );
    } else {
        pixels.extend(
            values
                .values()
                .as_chunks::<4>()
                .0
                .iter()
                .filter(|x| x[3] >= alpha_threshold)
                .map(|x| Rgb::new(x[0], x[1], x[2])),
        );
    }
    Ok(rgb_to_py(extract_color(&pixels)))
}
