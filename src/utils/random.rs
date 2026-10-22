use material_colors::utils::random::Random;
use pyo3::{pyclass, pymethods};

#[pyclass(name = "Random")]
pub struct PyRandom(pub Random);

#[pymethods]
impl PyRandom {
    #[new]
    pub fn new(seed: i64) -> Self {
        Self(Random::new(seed))
    }

    pub fn next_range(&mut self, range: i32) -> i32 {
        self.0.next_range(range)
    }
}
