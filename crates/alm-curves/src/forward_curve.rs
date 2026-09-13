use crate::{CurveId, YieldTermStructure};
use crate::interpolation::InterpolationMethod;
use alm_core::InterestRate;
use alm_time::day_count::DayCountConvention;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// A curve constructed from a series of forward rates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForwardCurve {
    pub id: CurveId,
    pub as_of_date: NaiveDate,
    pub day_count: DayCountConvention,
    pub interpolation: InterpolationMethod,
    /// Store (Time in years, Discount Factor) incrementally built from forward rates
    pub points: Vec<(f64, f64)>,
}

impl ForwardCurve {
    /// Constructs a forward curve from a series of consecutive forward rates.
    /// `inputs` is a list of (End Date, Forward Rate for period [previous_date, end_date]).
    /// The first period starts at `as_of_date`.
    pub fn new(
        id: CurveId,
        as_of_date: NaiveDate,
        day_count: DayCountConvention,
        interpolation: InterpolationMethod,
        mut inputs: Vec<(NaiveDate, InterestRate)>,
    ) -> Self {
        inputs.sort_by(|a, b| a.0.cmp(&b.0));
        let mut points = Vec::with_capacity(inputs.len());
        
        let mut current_date = as_of_date;
        let mut current_df = 1.0;
        
        for (end_date, rate) in inputs {
            if end_date <= current_date {
                continue;
            }
            
            let t = day_count.year_fraction(current_date, end_date);
            let period_df = rate.discount_factor(t);
            current_df *= period_df;
            
            let total_t = day_count.year_fraction(as_of_date, end_date);
            points.push((total_t, current_df));
            
            current_date = end_date;
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

impl YieldTermStructure for ForwardCurve {
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
