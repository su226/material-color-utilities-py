use material_colors::dynamic_color::{DeltaConstraint, ToneDeltaPair, TonePolarity};
use pyo3::prelude::*;

use crate::dynamic_color::PyDynamicColor;

/// Describes how to fulfill a tone delta pair constraint.
#[pyclass(name = "DeltaConstraint", from_py_object)]
#[derive(Debug, Clone, Copy)]
pub enum PyDeltaConstraint {
    /// The tone of `role_a` must be an exact delta away from the tone of
    /// `role_b`.
    Exact,
    /// The tonal distance of `role_a` and `role_b` must be at most delta.
    Nearer,
    /// The tonal distance of `role_a` and `role_b` must be at least delta.
    Farther,
}

impl From<PyDeltaConstraint> for DeltaConstraint {
    fn from(constraint: PyDeltaConstraint) -> Self {
        match constraint {
            PyDeltaConstraint::Exact => Self::Exact,
            PyDeltaConstraint::Nearer => Self::Nearer,
            PyDeltaConstraint::Farther => Self::Farther,
        }
    }
}

impl From<DeltaConstraint> for PyDeltaConstraint {
    fn from(constraint: DeltaConstraint) -> Self {
        match constraint {
            DeltaConstraint::Exact => Self::Exact,
            DeltaConstraint::Nearer => Self::Nearer,
            DeltaConstraint::Farther => Self::Farther,
        }
    }
}

/// Describes the relationship in lightness between two colors.
///
/// `RelativeDarker`/`RelativeLighter` are relative to the surface trend (white
/// in light mode, black in dark mode).
#[pyclass(name = "TonePolarity", from_py_object)]
#[derive(Debug, Clone, Copy)]
pub enum PyTonePolarity {
    Darker,
    Lighter,
    RelativeDarker,
    RelativeLighter,
}

impl From<PyTonePolarity> for TonePolarity {
    fn from(polarity: PyTonePolarity) -> Self {
        match polarity {
            PyTonePolarity::Darker => Self::Darker,
            PyTonePolarity::Lighter => Self::Lighter,
            PyTonePolarity::RelativeDarker => Self::RelativeDarker,
            PyTonePolarity::RelativeLighter => Self::RelativeLighter,
        }
    }
}

impl From<TonePolarity> for PyTonePolarity {
    fn from(polarity: TonePolarity) -> Self {
        match polarity {
            TonePolarity::Darker => Self::Darker,
            TonePolarity::Lighter => Self::Lighter,
            TonePolarity::RelativeDarker => Self::RelativeDarker,
            TonePolarity::RelativeLighter => Self::RelativeLighter,
        }
    }
}

/// Documents a constraint between two `DynamicColor`s, in which their tones
/// must have a certain distance from each other.
///
/// `polarity` describes `role_a` compared to `role_b`:
/// `ToneDeltaPair(A, B, 15, TonePolarity.Darker)` states that A's tone should
/// be at least 15 darker than B's.
#[pyclass(name = "ToneDeltaPair")]
#[derive(Clone)]
pub struct PyToneDeltaPair {
    pub role_a: PyDynamicColor,
    pub role_b: PyDynamicColor,
    pub delta: f64,
    pub polarity: PyTonePolarity,
    pub stay_together: bool,
    pub constraint: PyDeltaConstraint,
}

impl PyToneDeltaPair {
    /// Converts a Rust tone delta pair into its Python representation.
    ///
    /// Returns `None` for pairs involving user-defined colors, which cannot
    /// be represented by the material-role based [`PyDynamicColor`].
    pub fn from_rust(pair: ToneDeltaPair<'_>) -> Option<Self> {
        Some(Self {
            role_a: PyDynamicColor::from_dynamic(pair.role_a)?,
            role_b: PyDynamicColor::from_dynamic(pair.role_b)?,
            delta: pair.delta,
            polarity: pair.polarity.into(),
            stay_together: pair.stay_together,
            constraint: pair.constraint.into(),
        })
    }
}

#[pymethods]
impl PyToneDeltaPair {
    #[new]
    #[pyo3(signature = (role_a, role_b, delta, polarity, stay_together = true, constraint = PyDeltaConstraint::Exact))]
    pub fn new(
        role_a: &PyDynamicColor,
        role_b: &PyDynamicColor,
        delta: f64,
        polarity: PyTonePolarity,
        stay_together: bool,
        constraint: PyDeltaConstraint,
    ) -> Self {
        Self {
            role_a: role_a.clone(),
            role_b: role_b.clone(),
            delta,
            polarity,
            stay_together,
            constraint,
        }
    }

    #[getter]
    fn get_role_a(&self) -> PyDynamicColor {
        self.role_a.clone()
    }

    #[setter]
    fn set_role_a(&mut self, role_a: &PyDynamicColor) {
        self.role_a = role_a.clone();
    }

    #[getter]
    fn get_role_b(&self) -> PyDynamicColor {
        self.role_b.clone()
    }

    #[setter]
    fn set_role_b(&mut self, role_b: &PyDynamicColor) {
        self.role_b = role_b.clone();
    }

    #[getter]
    fn get_delta(&self) -> f64 {
        self.delta
    }

    #[setter]
    fn set_delta(&mut self, delta: f64) {
        self.delta = delta;
    }

    #[getter]
    fn get_polarity(&self) -> PyTonePolarity {
        self.polarity
    }

    #[setter]
    fn set_polarity(&mut self, polarity: PyTonePolarity) {
        self.polarity = polarity;
    }

    #[getter]
    fn get_stay_together(&self) -> bool {
        self.stay_together
    }

    #[setter]
    fn set_stay_together(&mut self, stay_together: bool) {
        self.stay_together = stay_together;
    }

    #[getter]
    fn get_constraint(&self) -> PyDeltaConstraint {
        self.constraint
    }

    #[setter]
    fn set_constraint(&mut self, constraint: PyDeltaConstraint) {
        self.constraint = constraint;
    }

    fn __repr__(&self) -> String {
        format!(
            "ToneDeltaPair(role_a={:?}, role_b={:?}, delta={}, polarity={:?})",
            self.role_a.name(),
            self.role_b.name(),
            self.delta,
            self.polarity
        )
    }
}
