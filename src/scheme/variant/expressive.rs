use pyo3::prelude::*;

use super::py_scheme_variant;

py_scheme_variant!(
    PySchemeExpressive,
    material_colors::scheme::variant::SchemeExpressive,
    Expressive,
    "SchemeExpressive"
);
