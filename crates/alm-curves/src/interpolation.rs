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

        let mut idx = 0;
        for i in 0..x_points.len() - 1 {
            if x >= x_points[i] && x <= x_points[i + 1] {
                idx = i;
                break;
            }
        }

        let x1 = x_points[idx];
        let x2 = x_points[idx + 1];
        let y1 = y_points[idx];
        let y2 = y_points[idx + 1];

        if (x2 - x1).abs() < 1e-10 {
            return y1;
        }

        match self {
            InterpolationMethod::Linear => {
                let weight = (x - x1) / (x2 - x1);
                y1 + weight * (y2 - y1)
            }
            InterpolationMethod::LogLinear => {
                // To avoid ln of zero or negative numbers:
                if y1 > 0.0 && y2 > 0.0 {
                    let weight = (x - x1) / (x2 - x1);
                    let ln_y = y1.ln() + weight * (y2.ln() - y1.ln());
                    ln_y.exp()
                } else {
                    // Fallback to linear if log is not possible
                    let weight = (x - x1) / (x2 - x1);
                    y1 + weight * (y2 - y1)
                }
            }
            InterpolationMethod::Step => y1,
            InterpolationMethod::FlatForward => {
                // Often flat forward on rates means log-linear on discount factors.
                // If we assume this interpolator is used directly on rates, 
                // FlatForward means rate is constant between nodes.
                // Let's implement it as a step function that evaluates to y1.
                y1
            }
            InterpolationMethod::MonotoneConvex => {
                // Simplified Monotonic Cubic Spline (Fritsch-Carlson) on the fly
                let n = x_points.len();
                let mut m = vec![0.0; n - 1];
                for i in 0..n - 1 {
                    m[i] = (y_points[i + 1] - y_points[i]) / (x_points[i + 1] - x_points[i]);
                }

                let mut d = vec![0.0; n];
                d[0] = m[0];
                d[n - 1] = m[n - 2];
                for i in 1..n - 1 {
                    if m[i - 1] * m[i] <= 0.0 {
                        d[i] = 0.0;
                    } else {
                        // Harmonic mean
                        d[i] = 2.0 / (1.0 / m[i - 1] + 1.0 / m[i]);
                    }
                }

                let h = x2 - x1;
                let t = (x - x1) / h;
                
                let h00 = 2.0 * t * t * t - 3.0 * t * t + 1.0;
                let h10 = t * t * t - 2.0 * t * t + t;
                let h01 = -2.0 * t * t * t + 3.0 * t * t;
                let h11 = t * t * t - t * t;

                let y = h00 * y1 + h10 * h * d[idx] + h01 * y2 + h11 * h * d[idx + 1];
                y
            }
        }
    }
}
