use material_colors::dynamic_color::Platform;
use pyo3::prelude::*;

#[pyclass(name = "Platform", from_py_object)]
#[derive(Clone)]
pub enum PyPlatform {
    Phone,
    Watch,
}

impl From<PyPlatform> for Platform {
    fn from(platform: PyPlatform) -> Self {
        match platform {
            PyPlatform::Phone => Self::Phone,
            PyPlatform::Watch => Self::Watch,
        }
    }
}

impl From<Platform> for PyPlatform {
    fn from(platform: Platform) -> Self {
        match platform {
            Platform::Phone => Self::Phone,
            Platform::Watch => Self::Watch,
        }
    }
}
