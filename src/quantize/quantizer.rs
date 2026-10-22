use indexmap::IndexMap;
use material_colors::quantize::QuantizerResult;
use pyo3::prelude::*;

use crate::color::{PyRgb, rgb_from_py, rgb_to_py};

/// The result of a quantization run.
#[pyclass(name = "QuantizerResult")]
pub struct PyQuantizerResult(pub QuantizerResult);

#[pymethods]
impl PyQuantizerResult {
    #[new]
    #[pyo3(signature = (color_to_count, input_pixel_to_cluster_pixel))]
    pub fn new(
        color_to_count: IndexMap<PyRgb, u32>,
        input_pixel_to_cluster_pixel: IndexMap<PyRgb, PyRgb>,
    ) -> Self {
        Self(QuantizerResult {
            color_to_count: color_to_count
                .into_iter()
                .map(|(rgb, count)| (rgb_from_py(rgb), count))
                .collect(),
            input_pixel_to_cluster_pixel: input_pixel_to_cluster_pixel
                .into_iter()
                .map(|(rgb, cluster)| (rgb_from_py(rgb), rgb_from_py(cluster)))
                .collect(),
        })
    }

    /// Maps each quantized color to the number of input pixels it represents.
    #[getter]
    fn get_color_to_count(&self) -> IndexMap<PyRgb, u32> {
        self.0
            .color_to_count
            .iter()
            .map(|(rgb, count)| (rgb_to_py(*rgb), *count))
            .collect()
    }

    #[setter]
    fn set_color_to_count(&mut self, color_to_count: IndexMap<PyRgb, u32>) {
        self.0.color_to_count = color_to_count
            .into_iter()
            .map(|(rgb, count)| (rgb_from_py(rgb), count))
            .collect();
    }

    /// Maps each input pixel to the quantized color it was assigned to.
    #[getter]
    fn get_input_pixel_to_cluster_pixel(&self) -> IndexMap<PyRgb, PyRgb> {
        self.0
            .input_pixel_to_cluster_pixel
            .iter()
            .map(|(rgb, cluster)| (rgb_to_py(*rgb), rgb_to_py(*cluster)))
            .collect()
    }

    #[setter]
    fn set_input_pixel_to_cluster_pixel(
        &mut self,
        input_pixel_to_cluster_pixel: IndexMap<PyRgb, PyRgb>,
    ) {
        self.0.input_pixel_to_cluster_pixel = input_pixel_to_cluster_pixel
            .into_iter()
            .map(|(rgb, cluster)| (rgb_from_py(rgb), rgb_from_py(cluster)))
            .collect();
    }
}
