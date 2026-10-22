pub mod cmf;
pub mod content;
pub mod expressive;
pub mod fidelity;
pub mod fruit_salad;
pub mod monochrome;
pub mod neutral;
pub mod rainbow;
pub mod tonal_spot;
pub mod vibrant;

/// Generates the Python wrapper for one of the predefined dynamic schemes.
macro_rules! py_scheme_variant {
    ($py_type:ident, $rust_type:path, $variant:ident, $name:literal) => {
        #[pyclass(name = $name)]
        pub struct $py_type(pub $rust_type);

        #[pymethods]
        impl $py_type {
            #[new]
            #[pyo3(signature = (source_color_hct, is_dark, contrast_level = None))]
            pub fn new(
                source_color_hct: &$crate::hct::PyHct,
                is_dark: bool,
                contrast_level: Option<f64>,
            ) -> Self {
                Self(<$rust_type>::new(
                    source_color_hct.0,
                    is_dark,
                    contrast_level,
                ))
            }

            /// Builds the scheme for any spec version and platform.
            #[staticmethod]
            #[pyo3(signature = (source_color_hct, is_dark, contrast_level, spec_version, platform))]
            pub fn with_spec(
                source_color_hct: &$crate::hct::PyHct,
                is_dark: bool,
                contrast_level: Option<f64>,
                spec_version: $crate::dynamic_color::color_spec::PySpecVersion,
                platform: $crate::dynamic_color::dynamic_scheme::PyPlatform,
            ) -> Self {
                Self(<$rust_type>::with_spec(
                    source_color_hct.0,
                    is_dark,
                    contrast_level,
                    spec_version.into(),
                    platform.into(),
                ))
            }

            /// One palette as built for spec 2021 on phone, light mode,
            /// standard contrast.
            #[staticmethod]
            pub fn palette(
                source_color_hct: &$crate::hct::PyHct,
                palette: $crate::palette::PyPalette,
            ) -> $crate::palette::tonal::PyTonalPalette {
                $crate::palette::tonal::PyTonalPalette(<$rust_type>::palette(
                    &source_color_hct.0,
                    &palette.into(),
                ))
            }

            /// The underlying dynamic scheme.
            #[getter]
            fn get_scheme(&self) -> $crate::dynamic_color::dynamic_scheme::PyDynamicScheme {
                $crate::dynamic_color::dynamic_scheme::PyDynamicScheme(self.0.scheme.clone())
            }

            /// The variant of this scheme.
            #[getter]
            fn get_variant(&self) -> $crate::dynamic_color::variant::PyVariant {
                $crate::dynamic_color::variant::PyVariant::$variant
            }

            /// Converts this scheme into a plain 49-color `Scheme`.
            fn to_scheme(&self) -> $crate::scheme::PyScheme {
                $crate::scheme::PyScheme(material_colors::scheme::Scheme::from(
                    self.0.scheme.clone(),
                ))
            }

            fn __repr__(&self) -> String {
                format!(
                    "{}({:?}, is_dark={})",
                    $name, self.0.scheme.variant, self.0.scheme.is_dark
                )
            }
        }
    };
}

pub(crate) use py_scheme_variant;
