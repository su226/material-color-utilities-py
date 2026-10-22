use pyo3::prelude::*;

use super::py_scheme_variant;

py_scheme_variant!(
    PySchemeRainbow,
    material_colors::scheme::variant::SchemeRainbow,
    Rainbow,
    "SchemeRainbow"
);
