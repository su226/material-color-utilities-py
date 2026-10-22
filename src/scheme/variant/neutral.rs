use pyo3::prelude::*;

use super::py_scheme_variant;

py_scheme_variant!(
    PySchemeNeutral,
    material_colors::scheme::variant::SchemeNeutral,
    Neutral,
    "SchemeNeutral"
);
