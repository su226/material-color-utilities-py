use pyo3::prelude::*;

use super::py_scheme_variant;

py_scheme_variant!(
    PySchemeMonochrome,
    material_colors::scheme::variant::SchemeMonochrome,
    Monochrome,
    "SchemeMonochrome"
);
