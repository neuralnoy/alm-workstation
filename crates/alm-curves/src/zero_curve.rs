use alm_core::{Compounding, InterestRate, NaiveDate};
use alm_time::day_count::DayCountConvention;
use serde::{Deserialize, Serialize};

/// A basic ZeroCurve that interpolates linearly on zero rates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroCurve {
    /// The base date for calculating distances.
    pub as_of_date: NaiveDate,
    /// The day count convention used by this curve.
    pub day_count: DayCountConvention,
    /// (Date, Zero Rate) points, assumed to be sorted by date.
    pub points: Vec<(NaiveDate, InterestRate)>,
}

impl ZeroCurve {
    pub fn new(
        as_of_date: NaiveDate,
        day_count: DayCountConvention,
        mut points: Vec<(NaiveDate, InterestRate)>,
    ) -> Self {
        points.sort_by(|a, b| a.0.cmp(&b.0));
        Self {
            as_of_date,
            day_count,
            points,
        }
    }

    /// Interpolate the zero rate for a given date.
    pub fn zero_rate(&self, target_date: NaiveDate) -> InterestRate {
        if self.points.is_empty() {
            return InterestRate::new(0.0, Compounding::Continuous);
        }

        if target_date <= self.points.first().unwrap().0 {
            return self.points.first().unwrap().1;
        }

        if target_date >= self.points.last().unwrap().0 {
            return self.points.last().unwrap().1;
        }

        // Linear interpolation
        for i in 0..self.points.len() - 1 {
            let (d1, r1) = self.points[i];
            let (d2, r2) = self.points[i + 1];

            if target_date >= d1 && target_date <= d2 {
                let total_days = (d2 - d1).num_days() as f64;
                let passed_days = (target_date - d1).num_days() as f64;
                let fraction = passed_days / total_days;

                let interpolated_val = r1.value + (r2.value - r1.value) * fraction;
                return InterestRate::new(interpolated_val, r1.compounding);
            }
        }

        self.points.last().unwrap().1
    }

    /// Get the discount factor for a given date.
    pub fn discount_factor(&self, target_date: NaiveDate) -> f64 {
        if target_date <= self.as_of_date {
            return 1.0;
        }
        let rate = self.zero_rate(target_date);
        let time = self.day_count.year_fraction(self.as_of_date, target_date);
        rate.discount_factor(time)
    }
}
