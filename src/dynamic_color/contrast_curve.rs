use material_colors::dynamic_color::ContrastCurve;
use pyo3::prelude::*;

/// A value that changes with the contrast level.
///
/// Usually represents the contrast requirements for a dynamic color on its
/// background. The four values correspond to values for contrast levels
/// -1.0, 0.0, 0.5, and 1.0, respectively.
#[pyclass(name = "ContrastCurve")]
#[derive(Clone, Copy)]
pub struct PyContrastCurve(pub ContrastCurve);

#[pymethods]
impl PyContrastCurve {
    #[new]
    pub fn new(low: f64, normal: f64, medium: f64, high: f64) -> Self {
        Self(ContrastCurve::new(low, normal, medium, high))
    }

    #[getter]
    fn get_low(&self) -> f64 {
        self.0.low
    }

    #[setter]
    fn set_low(&mut self, low: f64) {
        self.0.low = low;
    }

    #[getter]
    fn get_normal(&self) -> f64 {
        self.0.normal
    }

    #[setter]
    fn set_normal(&mut self, normal: f64) {
        self.0.normal = normal;
    }

    #[getter]
    fn get_medium(&self) -> f64 {
        self.0.medium
    }

    #[setter]
    fn set_medium(&mut self, medium: f64) {
        self.0.medium = medium;
    }

    #[getter]
    fn get_high(&self) -> f64 {
        self.0.high
    }

    #[setter]
    fn set_high(&mut self, high: f64) {
        self.0.high = high;
    }

    /// Returns the value at `contrast_level`: 0.0 is the default (normal),
    /// -1.0 the lowest and 1.0 the highest. Levels in between are linearly
    /// interpolated.
    fn get(&self, contrast_level: f64) -> f64 {
        self.0.get(contrast_level)
    }

    fn __repr__(&self) -> String {
        format!(
            "ContrastCurve(low={}, normal={}, medium={}, high={})",
            self.0.low, self.0.normal, self.0.medium, self.0.high
        )
    }
}
