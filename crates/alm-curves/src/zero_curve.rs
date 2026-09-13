use crate::{CurveId, YieldTermStructure};
use crate::interpolation::InterpolationMethod;
use alm_core::InterestRate;
use alm_time::day_count::DayCountConvention;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// A curve that interpolates zero rates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroCurve {
    pub id: CurveId,
    pub as_of_date: NaiveDate,
    pub day_count: DayCountConvention,
    pub interpolation: InterpolationMethod,
    /// Store continuous zero rates for interpolation
    /// (Time in years, continuous zero rate)
    pub points: Vec<(f64, f64)>,
}

impl ZeroCurve {
    pub fn new(
        id: CurveId,
        as_of_date: NaiveDate,
        day_count: DayCountConvention,
        interpolation: InterpolationMethod,
        mut inputs: Vec<(NaiveDate, InterestRate)>,
    ) -> Self {
        inputs.sort_by(|a, b| a.0.cmp(&b.0));
        let mut points = Vec::with_capacity(inputs.len());
        for (date, rate) in inputs {
            let t = day_count.year_fraction(as_of_date, date);
            if t < 0.0 {
                continue;
            }
            // Convert to continuous compounding for storage and interpolation
            let df = rate.discount_factor(t);
            let continuous_rate = if t > 0.0 { -df.ln() / t } else { rate.value };
            points.push((t, continuous_rate));
        }
        Self {
            id,
            as_of_date,
            day_count,
            interpolation,
            points,
        }
    }
}

impl YieldTermStructure for ZeroCurve {
    fn curve_id(&self) -> &CurveId {
        &self.id
    }

    fn reference_date(&self) -> NaiveDate {
        self.as_of_date
    }

    fn day_count(&self) -> DayCountConvention {
        self.day_count
    }

    fn discount_factor(&self, date: NaiveDate) -> f64 {
        if date <= self.as_of_date {
            return 1.0;
        }
        let t = self.day_count.year_fraction(self.as_of_date, date);
        let x_points: Vec<f64> = self.points.iter().map(|p| p.0).collect();
        let y_points: Vec<f64> = self.points.iter().map(|p| p.1).collect();
        
        let rate_val = self.interpolation.interpolate(t, &x_points, &y_points);
        (-rate_val * t).exp()
    }
}
