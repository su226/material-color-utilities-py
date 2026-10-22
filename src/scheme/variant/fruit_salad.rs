use pyo3::prelude::*;

use super::py_scheme_variant;

py_scheme_variant!(
    PySchemeFruitSalad,
    material_colors::scheme::variant::SchemeFruitSalad,
    FruitSalad,
    "SchemeFruitSalad"
);
