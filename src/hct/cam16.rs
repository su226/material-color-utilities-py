use material_colors::hct::{ViewingConditions, cam16::Cam16};
use pyo3::prelude::*;

use crate::{
    color::{PyRgb, PyXyz, rgb_from_py, rgb_to_py, xyz_from_py, xyz_to_py},
    hct::viewing_conditions::PyViewingConditions,
};

#[pyclass(name = "Cam16")]
pub struct PyCam16(pub Cam16);

#[pymethods]
impl PyCam16 {
    #[new]
    fn new(
        hue: f64,
        chroma: f64,
        lightness: f64,
        brightness: f64,
        colorfulness: f64,
        saturation: f64,
        jstar: f64,
        astar: f64,
        bstar: f64,
    ) -> Self {
        Self(Cam16 {
            hue,
            chroma,
            lightness,
            brightness,
            colorfulness,
            saturation,
            jstar,
            astar,
            bstar,
        })
    }

    #[getter]
    fn get_hue(&self) -> f64 {
        self.0.hue
    }

    #[setter]
    fn set_hue(&mut self, hue: f64) {
        self.0.hue = hue;
    }

    #[getter]
    fn get_chroma(&self) -> f64 {
        self.0.chroma
    }

    #[setter]
    fn set_chroma(&mut self, chroma: f64) {
        self.0.chroma = chroma;
    }

    #[getter]
    fn get_lightness(&self) -> f64 {
        self.0.lightness
    }

    #[setter]
    fn set_lightness(&mut self, lightness: f64) {
        self.0.lightness = lightness;
    }

    #[getter]
    fn get_brightness(&self) -> f64 {
        self.0.brightness
    }

    #[setter]
    fn set_brightness(&mut self, brightness: f64) {
        self.0.brightness = brightness;
    }

    #[getter]
    fn get_colorfulness(&self) -> f64 {
        self.0.colorfulness
    }

    #[setter]
    fn set_colorfulness(&mut self, colorfulness: f64) {
        self.0.colorfulness = colorfulness;
    }

    #[getter]
    fn get_saturation(&self) -> f64 {
        self.0.saturation
    }

    #[setter]
    fn set_saturation(&mut self, saturation: f64) {
        self.0.saturation = saturation;
    }

    #[getter]
    fn get_jstar(&self) -> f64 {
        self.0.jstar
    }

    #[setter]
    fn set_jstar(&mut self, jstar: f64) {
        self.0.jstar = jstar;
    }

    #[getter]
    fn get_astar(&self) -> f64 {
        self.0.astar
    }

    #[setter]
    fn set_astar(&mut self, astar: f64) {
        self.0.astar = astar;
    }

    #[getter]
    fn get_bstar(&self) -> f64 {
        self.0.bstar
    }

    #[setter]
    fn set_bstar(&mut self, bstar: f64) {
        self.0.bstar = bstar;
    }

    pub fn distance(&self, other: &Self) -> f64 {
        self.0.distance(&other.0)
    }

    #[staticmethod]
    #[pyo3(signature = (rgb, viewing_conditions = None))]
    pub fn from_rgb(rgb: PyRgb, viewing_conditions: Option<&PyViewingConditions>) -> Self {
        if let Some(viewing_conditions) = viewing_conditions {
            Self(Cam16::from_rgb_in_viewing_conditions(
                rgb_from_py(rgb),
                &viewing_conditions.0,
            ))
        } else {
            Self(rgb_from_py(rgb).into())
        }
    }

    #[staticmethod]
    #[pyo3(signature = (value, viewing_conditions = None))]
    pub fn from_xyz(value: PyXyz, viewing_conditions: Option<&PyViewingConditions>) -> Self {
        if let Some(viewing_conditions) = viewing_conditions {
            Self(Cam16::from_xyz_in_viewing_conditions(
                xyz_from_py(value),
                &viewing_conditions.0,
            ))
        } else {
            Self(Cam16::from_xyz_in_viewing_conditions(
                xyz_from_py(value),
                &ViewingConditions::s_rgb(),
            ))
        }
    }

    #[staticmethod]
    #[pyo3(signature = (lightness, chroma, hue, viewing_conditions = None))]
    pub fn from_jch(
        lightness: f64,
        chroma: f64,
        hue: f64,
        viewing_conditions: Option<&PyViewingConditions>,
    ) -> Self {
        if let Some(viewing_conditions) = viewing_conditions {
            Self(Cam16::from_jch_in_viewing_conditions(
                lightness,
                chroma,
                hue,
                &viewing_conditions.0,
            ))
        } else {
            Self(Cam16::from_jch(lightness, chroma, hue))
        }
    }

    #[staticmethod]
    #[pyo3(signature = (jstar, astar, bstar, viewing_conditions = None))]
    pub fn from_ucs(
        jstar: f64,
        astar: f64,
        bstar: f64,
        viewing_conditions: Option<&PyViewingConditions>,
    ) -> Self {
        if let Some(viewing_conditions) = viewing_conditions {
            Self(Cam16::from_ucs_in_viewing_conditions(
                jstar,
                astar,
                bstar,
                &viewing_conditions.0,
            ))
        } else {
            Self(Cam16::from_ucs(jstar, astar, bstar))
        }
    }

    #[pyo3(signature = (viewing_conditions = None))]
    pub fn to_rgb(&self, viewing_conditions: Option<&PyViewingConditions>) -> PyRgb {
        if let Some(viewing_conditions) = viewing_conditions {
            rgb_to_py(self.0.viewed(&viewing_conditions.0))
        } else {
            rgb_to_py(self.0.viewed(&ViewingConditions::s_rgb()))
        }
    }

    #[pyo3(signature = (viewing_conditions = None))]
    pub fn to_xyz(&self, viewing_conditions: Option<&PyViewingConditions>) -> PyXyz {
        if let Some(viewing_conditions) = viewing_conditions {
            xyz_to_py(self.0.xyz_in_viewing_conditions(&viewing_conditions.0))
        } else {
            xyz_to_py(
                self.0
                    .xyz_in_viewing_conditions(&ViewingConditions::s_rgb()),
            )
        }
    }
}
