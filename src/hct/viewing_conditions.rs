use material_colors::hct::viewing_conditions::ViewingConditions;
use pyo3::prelude::*;

#[pyclass(name = "ViewingConditions")]
pub struct PyViewingConditions(pub ViewingConditions);

#[pymethods]
impl PyViewingConditions {
    #[getter]
    fn get_white_point(&self) -> [f64; 3] {
        self.0.white_point
    }

    #[setter]
    fn set_white_point(&mut self, white_point: [f64; 3]) {
        self.0.white_point = white_point;
    }

    #[getter]
    fn get_adapting_luminance(&self) -> f64 {
        self.0.adapting_luminance
    }

    #[setter]
    fn set_adapting_luminance(&mut self, adapting_luminance: f64) {
        self.0.adapting_luminance = adapting_luminance;
    }

    #[getter]
    fn get_background_lstar(&self) -> f64 {
        self.0.background_lstar
    }

    #[setter]
    fn set_background_lstar(&mut self, background_lstar: f64) {
        self.0.background_lstar = background_lstar;
    }

    #[getter]
    fn get_surround(&self) -> f64 {
        self.0.surround
    }

    #[setter]
    fn set_surround(&mut self, surround: f64) {
        self.0.surround = surround;
    }

    #[getter]
    fn get_discounting_illuminant(&self) -> bool {
        self.0.discounting_illuminant
    }

    #[setter]
    fn set_discounting_illuminant(&mut self, discounting_illuminant: bool) {
        self.0.discounting_illuminant = discounting_illuminant;
    }

    #[getter]
    fn get_background_ytowhite_point_y(&self) -> f64 {
        self.0.background_ytowhite_point_y
    }

    #[setter]
    fn set_background_ytowhite_point_y(&mut self, background_ytowhite_point_y: f64) {
        self.0.background_ytowhite_point_y = background_ytowhite_point_y;
    }

    #[getter]
    fn get_aw(&self) -> f64 {
        self.0.aw
    }

    #[setter]
    fn set_aw(&mut self, aw: f64) {
        self.0.aw = aw;
    }

    #[getter]
    fn get_nbb(&self) -> f64 {
        self.0.nbb
    }

    #[setter]
    fn set_nbb(&mut self, nbb: f64) {
        self.0.nbb = nbb;
    }

    #[getter]
    fn get_ncb(&self) -> f64 {
        self.0.ncb
    }

    #[setter]
    fn set_ncb(&mut self, ncb: f64) {
        self.0.ncb = ncb;
    }

    #[getter]
    fn get_c(&self) -> f64 {
        self.0.c
    }

    #[setter]
    fn set_c(&mut self, c: f64) {
        self.0.c = c;
    }

    #[getter]
    fn get_n_c(&self) -> f64 {
        self.0.n_c
    }

    #[setter]
    fn set_n_c(&mut self, n_c: f64) {
        self.0.n_c = n_c;
    }

    #[getter]
    fn get_drgb_inverse(&self) -> [f64; 3] {
        self.0.drgb_inverse
    }

    #[setter]
    fn set_drgb_inverse(&mut self, drgb_inverse: [f64; 3]) {
        self.0.drgb_inverse = drgb_inverse;
    }

    #[getter]
    fn get_rgb_d(&self) -> [f64; 3] {
        self.0.rgb_d
    }

    #[setter]
    fn set_rgb_d(&mut self, rgb_d: [f64; 3]) {
        self.0.rgb_d = rgb_d;
    }

    #[getter]
    fn get_fl(&self) -> f64 {
        self.0.fl
    }

    #[setter]
    fn set_fl(&mut self, fl: f64) {
        self.0.fl = fl;
    }

    #[getter]
    fn get_f_lroot(&self) -> f64 {
        self.0.f_lroot
    }

    #[setter]
    fn set_f_lroot(&mut self, f_lroot: f64) {
        self.0.f_lroot = f_lroot;
    }

    #[getter]
    fn get_z(&self) -> f64 {
        self.0.z
    }

    #[setter]
    fn set_z(&mut self, z: f64) {
        self.0.z = z;
    }

    #[new]
    #[pyo3(signature = (white_point = None, adapting_luminance = None, background_lstar = None, surround = None, discounting_illuminant = None))]
    pub fn new(
        white_point: Option<[f64; 3]>,
        adapting_luminance: Option<f64>,
        background_lstar: Option<f64>,
        surround: Option<f64>,
        discounting_illuminant: Option<bool>,
    ) -> Self {
        Self(ViewingConditions::make(
            white_point,
            adapting_luminance,
            background_lstar,
            surround,
            discounting_illuminant,
        ))
    }

    #[staticmethod]
    pub fn from_raw(
        white_point: [f64; 3],
        adapting_luminance: f64,
        background_lstar: f64,
        surround: f64,
        discounting_illuminant: bool,
        background_ytowhite_point_y: f64,
        aw: f64,
        nbb: f64,
        ncb: f64,
        c: f64,
        n_c: f64,
        drgb_inverse: [f64; 3],
        rgb_d: [f64; 3],
        fl: f64,
        f_lroot: f64,
        z: f64,
    ) -> Self {
        Self(ViewingConditions {
            white_point,
            adapting_luminance,
            background_lstar,
            surround,
            discounting_illuminant,
            background_ytowhite_point_y,
            aw,
            nbb,
            ncb,
            c,
            n_c,
            drgb_inverse,
            rgb_d,
            fl,
            f_lroot,
            z,
        })
    }
}
