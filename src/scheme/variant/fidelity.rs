use pyo3::prelude::*;

use super::py_scheme_variant;

py_scheme_variant!(
    PySchemeFidelity,
    material_colors::scheme::variant::SchemeFidelity,
    Fidelity,
    "SchemeFidelity"
);
