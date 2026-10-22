use pyo3::prelude::*;

use super::color_spec::py_color_spec_version;

py_color_spec_version!(PyColorSpec2025, "ColorSpec2025", Spec2025);
