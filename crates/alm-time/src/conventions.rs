use crate::business_day::BusinessDayConvention;
use crate::day_count::DayCountConvention;

/// A struct that groups together common conventions used in financial time calculations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Conventions {
    pub business_day_convention: BusinessDayConvention,
    pub day_count_convention: DayCountConvention,
}

impl Conventions {
    pub fn new(
        business_day_convention: BusinessDayConvention,
        day_count_convention: DayCountConvention,
    ) -> Self {
        Self {
            business_day_convention,
            day_count_convention,
        }
    }

    /// Returns default conventions commonly used for simple calculations (NoAdjustment, Actual/365).
    pub fn default() -> Self {
        Self {
            business_day_convention: BusinessDayConvention::NoAdjustment,
            day_count_convention: DayCountConvention::Actual365,
        }
    }
}

impl Default for Conventions {
    fn default() -> Self {
        Self::default()
    }
}
