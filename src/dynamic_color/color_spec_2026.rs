use pyo3::prelude::*;

use super::color_spec::py_color_spec_version;

py_color_spec_version!(PyColorSpec2026, "ColorSpec2026", Spec2026);
