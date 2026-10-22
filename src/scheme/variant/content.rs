use pyo3::prelude::*;

use super::py_scheme_variant;

py_scheme_variant!(
    PySchemeContent,
    material_colors::scheme::variant::SchemeContent,
    Content,
    "SchemeContent"
);
