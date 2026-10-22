use pyo3::prelude::*;

use super::color_spec::py_color_spec_version;

py_color_spec_version!(PyColorSpec2021, "ColorSpec2021", Spec2021);
