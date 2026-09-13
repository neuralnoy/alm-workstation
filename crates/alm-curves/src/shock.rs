use crate::{YieldCurve, ZeroCurve, DiscountCurve, ForwardCurve};
use serde::{Deserialize, Serialize};

/// Represents a shock scenario to be applied to an interest rate curve.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CurveShock {
    /// Parallel shift applied to all rates. Value is in decimal (e.g., 0.01 for 100 bps).
    ParallelShift(f64),
    /// Twist around a pivot time (in years), applying `slope * (t - pivot_time)`.
    Twist { pivot_time: f64, slope: f64 },
}

pub trait CurveShift {
    /// Applies a shock and returns a new shifted curve without mutating the original.
    fn apply_shock(&self, shock: &CurveShock) -> Self;
}

impl CurveShift for ZeroCurve {
    fn apply_shock(&self, shock: &CurveShock) -> Self {
        let mut new_points = Vec::with_capacity(self.points.len());
        for &(t, r) in &self.points {
            let shift = match shock {
                CurveShock::ParallelShift(s) => *s,
                CurveShock::Twist { pivot_time, slope } => slope * (t - pivot_time),
            };
            new_points.push((t, r + shift));
        }
        
        let mut new_curve = self.clone();
        new_curve.points = new_points;
        new_curve
    }
}

impl CurveShift for DiscountCurve {
    fn apply_shock(&self, shock: &CurveShock) -> Self {
        let mut new_points = Vec::with_capacity(self.points.len());
        for &(t, df) in &self.points {
            if t == 0.0 {
                new_points.push((t, df));
                continue;
            }
            // Derive continuous zero rate, shift it, and convert back to discount factor.
            let r = -df.ln() / t;
            let shift = match shock {
                CurveShock::ParallelShift(s) => *s,
                CurveShock::Twist { pivot_time, slope } => slope * (t - pivot_time),
            };
            let new_r = r + shift;
            let new_df = (-new_r * t).exp();
            new_points.push((t, new_df));
        }
        
        let mut new_curve = self.clone();
        new_curve.points = new_points;
        new_curve
    }
}

impl CurveShift for ForwardCurve {
    fn apply_shock(&self, shock: &CurveShock) -> Self {
        // Since ForwardCurve internally stores Discount Factors incrementally,
        // we can shift it the same way as a DiscountCurve for simplicity,
        // which shifts the implied zero rates.
        let mut new_points = Vec::with_capacity(self.points.len());
        for &(t, df) in &self.points {
            if t == 0.0 {
                new_points.push((t, df));
                continue;
            }
            let r = -df.ln() / t;
            let shift = match shock {
                CurveShock::ParallelShift(s) => *s,
                CurveShock::Twist { pivot_time, slope } => slope * (t - pivot_time),
            };
            let new_r = r + shift;
            let new_df = (-new_r * t).exp();
            new_points.push((t, new_df));
        }
        
        let mut new_curve = self.clone();
        new_curve.points = new_points;
        new_curve
    }
}

impl CurveShift for YieldCurve {
    fn apply_shock(&self, shock: &CurveShock) -> Self {
        match self {
            YieldCurve::Zero(c) => YieldCurve::Zero(c.apply_shock(shock)),
            YieldCurve::Discount(c) => YieldCurve::Discount(c.apply_shock(shock)),
            YieldCurve::Forward(c) => YieldCurve::Forward(c.apply_shock(shock)),
        }
    }
}
