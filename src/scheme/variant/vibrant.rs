use pyo3::prelude::*;

use super::py_scheme_variant;

py_scheme_variant!(
    PySchemeVibrant,
    material_colors::scheme::variant::SchemeVibrant,
    Vibrant,
    "SchemeVibrant"
);
