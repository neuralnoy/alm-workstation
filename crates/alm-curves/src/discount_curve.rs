use crate::{CurveId, YieldTermStructure};
use alm_math::interpolation::InterpolationMethod;
use alm_time::day_count::DayCountConvention;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// A curve that interpolates discount factors directly.
/// Typically uses LogLinear interpolation which is equivalent to flat forward rates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscountCurve {
    pub id: CurveId,
    pub as_of_date: NaiveDate,
    pub day_count: DayCountConvention,
    pub interpolation: InterpolationMethod,
    /// Store (Time in years, Discount Factor)
    pub points: Vec<(f64, f64)>,
}

impl DiscountCurve {
    pub fn new(
        id: CurveId,
        as_of_date: NaiveDate,
        day_count: DayCountConvention,
        interpolation: InterpolationMethod,
        mut inputs: Vec<(NaiveDate, f64)>,
    ) -> Self {
        inputs.sort_by(|a, b| a.0.cmp(&b.0));
        let mut points = Vec::with_capacity(inputs.len());
        for (date, df) in inputs {
            let t = day_count.year_fraction(as_of_date, date);
            if t < 0.0 {
                continue;
            }
            points.push((t, df));
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

impl YieldTermStructure for DiscountCurve {
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
        
        self.interpolation.interpolate(t, &x_points, &y_points)
    }
}
