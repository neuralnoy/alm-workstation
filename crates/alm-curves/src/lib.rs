pub mod bootstrap;
pub mod curve_set;
pub mod discount_curve;
pub mod forward_curve;
pub mod interpolation;
pub mod ois_curve;
pub mod shock;
pub mod swap_curve;
pub mod yield_curve;
pub mod zero_curve;

use alm_core::{Compounding, InterestRate};
use alm_time::day_count::DayCountConvention;
use alm_time::frequency::Frequency;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fmt;

pub use zero_curve::ZeroCurve;
pub use discount_curve::DiscountCurve;
pub use forward_curve::ForwardCurve;
pub use yield_curve::YieldCurve;
pub use curve_set::CurveSet;

/// Identifier for a curve (e.g., "USD-OIS", "EUR-EURIBOR-6M").
/// This is the standard approach in banking for identifying market data curves.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CurveId(pub String);

impl CurveId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl fmt::Display for CurveId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A foundational trait that all interest rate curves will implement.
pub trait YieldTermStructure {
    /// Returns the curve identifier.
    fn curve_id(&self) -> &CurveId;

    /// Returns the reference date (as-of date) of the curve.
    fn reference_date(&self) -> NaiveDate;

    /// Returns the day count convention used by the curve.
    fn day_count(&self) -> DayCountConvention;

    /// Calculates the discount factor for a given target date.
    fn discount_factor(&self, date: NaiveDate) -> f64;

    /// Calculates the zero rate for a given target date, matching the specified compounding and frequency.
    fn zero_rate(
        &self,
        date: NaiveDate,
        compounding: Compounding,
        frequency: Frequency,
    ) -> InterestRate {
        let df = self.discount_factor(date);
        let t = self.day_count().year_fraction(self.reference_date(), date);
        
        if t <= 0.0 {
            return InterestRate::new(0.0, compounding, frequency);
        }
        
        let val = match compounding {
            Compounding::Continuous => -df.ln() / t,
            Compounding::Simple => (1.0 / df - 1.0) / t,
            Compounding::Compounded => {
                let m = frequency.periods_per_year();
                if m.is_infinite() {
                    -df.ln() / t
                } else if m > 0.0 {
                    m * (df.powf(-1.0 / (m * t)) - 1.0)
                } else {
                    df.powf(-1.0 / t) - 1.0
                }
            }
        };
        InterestRate::new(val, compounding, frequency)
    }

    /// Calculates the forward rate between two dates, matching the specified compounding and frequency.
    fn forward_rate(
        &self,
        d1: NaiveDate,
        d2: NaiveDate,
        compounding: Compounding,
        frequency: Frequency,
    ) -> InterestRate {
        if d1 >= d2 {
            return InterestRate::new(0.0, compounding, frequency);
        }
        let df1 = self.discount_factor(d1);
        let df2 = self.discount_factor(d2);
        let t = self.day_count().year_fraction(d1, d2);
        
        if t <= 0.0 {
            return InterestRate::new(0.0, compounding, frequency);
        }

        let forward_df = df2 / df1;
        let val = match compounding {
            Compounding::Continuous => -forward_df.ln() / t,
            Compounding::Simple => (1.0 / forward_df - 1.0) / t,
            Compounding::Compounded => {
                let m = frequency.periods_per_year();
                if m.is_infinite() {
                    -forward_df.ln() / t
                } else if m > 0.0 {
                    m * (forward_df.powf(-1.0 / (m * t)) - 1.0)
                } else {
                    forward_df.powf(-1.0 / t) - 1.0
                }
            }
        };
        InterestRate::new(val, compounding, frequency)
    }
}
