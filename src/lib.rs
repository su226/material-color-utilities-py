#![allow(deprecated)]
#![allow(clippy::too_many_arguments)]

use pyo3::prelude::*;

mod blend;
mod color;
mod contrast;
mod dislike;
mod dynamic_color;
mod hct;
mod image;
mod palette;
mod pixels;
mod quantize;
mod scheme;
mod score;
mod temperature;
mod theme;
mod utils;

#[pymodule]
mod _core {
    #[pymodule_export]
    use crate::blend::py_harmonize;

    #[pymodule_export]
    use crate::blend::py_hct_hue;

    #[pymodule_export]
    use crate::blend::py_cam16_ucs;

    #[pymodule_export]
    const SRGB_TO_XYZ: [[f64; 3]; 3] = material_colors::color::SRGB_TO_XYZ;

    #[pymodule_export]
    const XYZ_TO_SRGB: [[f64; 3]; 3] = material_colors::color::SRGB_TO_XYZ;

    #[pymodule_export]
    const WHITE_POINT_D65: [f64; 3] = material_colors::color::WHITE_POINT_D65;

    #[pymodule_export]
    use crate::color::py_linear_rgb_to_rgb;

    #[pymodule_export]
    use crate::color::py_xyz_to_rgb;

    #[pymodule_export]
    use crate::color::py_rgb_to_xyz;

    #[pymodule_export]
    use crate::color::py_lab_to_rgb;

    #[pymodule_export]
    use crate::color::py_rgb_to_lab;

    #[pymodule_export]
    use crate::color::py_hex_to_rgb;

    #[pymodule_export]
    use crate::color::py_int_to_rgb;

    #[pymodule_export]
    use crate::color::py_lstar_to_rgb;

    #[pymodule_export]
    use crate::color::py_rgb_to_lstar;

    #[pymodule_export]
    use crate::color::py_rgb_to_int;

    #[pymodule_export]
    use crate::color::py_rgb_to_argb_int;

    #[pymodule_export]
    use crate::color::py_rgb_to_hex;

    #[pymodule_export]
    use crate::color::py_y_from_lstar;

    #[pymodule_export]
    use crate::color::py_lstar_from_y;

    #[pymodule_export]
    use crate::color::py_linearized;

    #[pymodule_export]
    use crate::color::py_delinearized;

    #[pymodule_export]
    use crate::contrast::py_ratio_of_tones;

    #[pymodule_export]
    use crate::contrast::py_lighter;

    #[pymodule_export]
    use crate::contrast::py_darker;

    #[pymodule_export]
    use crate::contrast::py_lighter_unsafe;

    #[pymodule_export]
    use crate::contrast::py_darker_unsafe;

    #[pymodule_export]
    use crate::dislike::py_is_disliked;

    #[pymodule_export]
    use crate::dislike::py_fix_if_disliked;

    #[pymodule_export]
    use crate::dynamic_color::color_spec::PyColorSpec;

    #[pymodule_export]
    use crate::dynamic_color::color_spec::PySpecVersion;

    #[pymodule_export]
    use crate::dynamic_color::color_spec_2021::PyColorSpec2021;

    #[pymodule_export]
    use crate::dynamic_color::color_spec_2025::PyColorSpec2025;

    #[pymodule_export]
    use crate::dynamic_color::color_spec_2026::PyColorSpec2026;

    #[pymodule_export]
    use crate::dynamic_color::contrast_curve::PyContrastCurve;

    #[pymodule_export]
    use crate::dynamic_color::dynamic_scheme::PyDynamicScheme;

    #[pymodule_export]
    use crate::dynamic_color::dynamic_scheme::PyPlatform;

    #[pymodule_export]
    use crate::dynamic_color::dynamic_scheme::PySchemeResolver;

    #[pymodule_export]
    use crate::dynamic_color::material_dynamic_colors::PyMaterialDynamicColors;

    #[pymodule_export]
    use crate::dynamic_color::tone_delta_pair::{
        PyDeltaConstraint, PyToneDeltaPair, PyTonePolarity,
    };

    #[pymodule_export]
    use crate::dynamic_color::variant::PyVariant;

    #[pymodule_export]
    use crate::dynamic_color::PyArgbColor;

    #[pymodule_export]
    use crate::dynamic_color::PyDynamicColor;

    #[pymodule_export]
    use crate::hct::PyHct;

    #[pymodule_export]
    use crate::hct::cam16::PyCam16;

    #[pymodule_export]
    use crate::hct::solver::py_solve_to_cam;

    #[pymodule_export]
    use crate::hct::solver::py_solve_to_rgb;

    #[pymodule_export]
    use crate::hct::viewing_conditions::PyViewingConditions;

    #[pymodule_export]
    use crate::palette::PyPalette;

    #[pymodule_export]
    use crate::palette::core::PyCorePalette;

    #[pymodule_export]
    use crate::palette::core::PyCorePalettes;

    #[pymodule_export]
    use crate::palette::tonal::PyTonalPalette;

    #[pymodule_export]
    use crate::quantize::point_provider_lab::PyPointProviderLab;

    #[pymodule_export]
    use crate::quantize::quantizer::PyQuantizerResult;

    #[pymodule_export]
    use crate::quantize::quantizer_celebi::py_quantize_celebi_raw;

    #[pymodule_export]
    use crate::quantize::quantizer_celebi::py_quantize_celebi_rgb;

    #[pymodule_export]
    use crate::quantize::quantizer_map::py_quantize_map_raw;

    #[pymodule_export]
    use crate::quantize::quantizer_map::py_quantize_map_rgb;

    #[pymodule_export]
    use crate::quantize::quantizer_wsmeans::py_quantize_wsmeans_raw;

    #[pymodule_export]
    use crate::quantize::quantizer_wsmeans::py_quantize_wsmeans_rgb;

    #[pymodule_export]
    use crate::quantize::quantizer_wu::py_quantize_wu_raw;

    #[pymodule_export]
    use crate::quantize::quantizer_wu::py_quantize_wu_rgb;

    #[pymodule_export]
    use crate::scheme::PyScheme;

    #[pymodule_export]
    use crate::scheme::variant::cmf::PySchemeCmf;

    #[pymodule_export]
    use crate::scheme::variant::content::PySchemeContent;

    #[pymodule_export]
    use crate::scheme::variant::expressive::PySchemeExpressive;

    #[pymodule_export]
    use crate::scheme::variant::fidelity::PySchemeFidelity;

    #[pymodule_export]
    use crate::scheme::variant::fruit_salad::PySchemeFruitSalad;

    #[pymodule_export]
    use crate::scheme::variant::monochrome::PySchemeMonochrome;

    #[pymodule_export]
    use crate::scheme::variant::neutral::PySchemeNeutral;

    #[pymodule_export]
    use crate::scheme::variant::rainbow::PySchemeRainbow;

    #[pymodule_export]
    use crate::scheme::variant::tonal_spot::PySchemeTonalSpot;

    #[pymodule_export]
    use crate::scheme::variant::vibrant::PySchemeVibrant;

    #[pymodule_export]
    use crate::score::py_score;

    #[pymodule_export]
    use crate::temperature::PyTemperatureCache;

    #[pymodule_export]
    use crate::theme::PyPalettes;

    #[pymodule_export]
    use crate::theme::PySchemes;

    #[pymodule_export]
    use crate::theme::PyTheme;

    #[pymodule_export]
    use crate::theme::PyThemeBuilder;

    #[pymodule_export]
    use crate::image::py_extract_color_raw;

    #[pymodule_export]
    use crate::image::py_extract_color_rgb;

    #[pymodule_export]
    use crate::pixels::PyPixelFormat;

    #[pymodule_export]
    use crate::utils::math::py_signum;

    #[pymodule_export]
    use crate::utils::math::py_lerp;

    #[pymodule_export]
    use crate::utils::math::py_sanitize_degrees_int;

    #[pymodule_export]
    use crate::utils::math::py_sanitize_degrees_double;

    #[pymodule_export]
    use crate::utils::math::py_rotate_direction;

    #[pymodule_export]
    use crate::utils::math::py_difference_degrees;

    #[pymodule_export]
    use crate::utils::math::py_matrix_multiply;

    #[pymodule_export]
    use crate::utils::random::PyRandom;
}
