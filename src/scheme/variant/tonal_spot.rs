use pyo3::prelude::*;

use super::py_scheme_variant;

py_scheme_variant!(
    PySchemeTonalSpot,
    material_colors::scheme::variant::SchemeTonalSpot,
    TonalSpot,
    "SchemeTonalSpot"
);
