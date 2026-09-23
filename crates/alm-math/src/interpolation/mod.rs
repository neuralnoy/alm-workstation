pub mod linear;
pub mod cubic_spline;
pub mod monotone;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterpolationMethod {
    Linear,
    LogLinear,
    Step,
    FlatForward,
    MonotoneConvex,
}

impl InterpolationMethod {
    /// Interpolates a value at `x` given a set of `x_points` and `y_points`.
    /// `x_points` must be strictly increasing.
    pub fn interpolate(&self, x: f64, x_points: &[f64], y_points: &[f64]) -> f64 {
        if x_points.is_empty() || y_points.is_empty() {
            return 0.0;
        }
        if x_points.len() == 1 {
            return y_points[0];
        }

        if x <= x_points[0] {
            return y_points[0];
        }
        if x >= *x_points.last().unwrap() {
            return *y_points.last().unwrap();
        }

        // Find the index using binary search for O(log N) performance
        let idx = x_points.partition_point(|&p| p <= x) - 1;

        let x1 = x_points[idx];
        let x2 = x_points[idx + 1];
        let y1 = y_points[idx];
        let y2 = y_points[idx + 1];

        match self {
            InterpolationMethod::Linear => linear::interpolate_linear(x, x1, x2, y1, y2),
            InterpolationMethod::LogLinear => linear::interpolate_log_linear(x, x1, x2, y1, y2),
            InterpolationMethod::Step => y1,
            InterpolationMethod::FlatForward => y1,
            InterpolationMethod::MonotoneConvex => monotone::interpolate_monotone_convex(x, x_points, y_points, idx),
        }
    }
}
