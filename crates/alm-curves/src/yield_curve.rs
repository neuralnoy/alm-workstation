use crate::{CurveId, DiscountCurve, ForwardCurve, YieldTermStructure, ZeroCurve};
use alm_time::day_count::DayCountConvention;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// A unified wrapper for different types of interest rate curves.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum YieldCurve {
    Zero(ZeroCurve),
    Discount(DiscountCurve),
    Forward(ForwardCurve),
}

impl YieldTermStructure for YieldCurve {
    fn curve_id(&self) -> &CurveId {
        match self {
            YieldCurve::Zero(c) => &c.id,
            YieldCurve::Discount(c) => &c.id,
            YieldCurve::Forward(c) => &c.id,
        }
    }

    fn reference_date(&self) -> NaiveDate {
        match self {
            YieldCurve::Zero(c) => c.reference_date(),
            YieldCurve::Discount(c) => c.reference_date(),
            YieldCurve::Forward(c) => c.reference_date(),
        }
    }

    fn day_count(&self) -> DayCountConvention {
        match self {
            YieldCurve::Zero(c) => c.day_count(),
            YieldCurve::Discount(c) => c.day_count(),
            YieldCurve::Forward(c) => c.day_count(),
        }
    }

    fn discount_factor(&self, date: NaiveDate) -> f64 {
        match self {
            YieldCurve::Zero(c) => c.discount_factor(date),
            YieldCurve::Discount(c) => c.discount_factor(date),
            YieldCurve::Forward(c) => c.discount_factor(date),
        }
    }
}
