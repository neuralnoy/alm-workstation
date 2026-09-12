use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

/// Defines how days in an accrual period are counted for interest rate calculations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DayCountConvention {
    /// 30/360: Assumes 30 days per month and 360 days per year.
    Thirty360,
    /// Actual/Actual: Actual days in the period divided by actual days in the year.
    ActualActual,
    /// Actual/365: Actual days in the period divided by 365.
    Actual365,
    /// Actual/360: Actual days in the period divided by 360.
    Actual360,
}

impl DayCountConvention {
    /// Calculates the number of days between two dates based on the convention.
    pub fn day_count(&self, start_date: NaiveDate, end_date: NaiveDate) -> i64 {
        match self {
            DayCountConvention::Thirty360 => {
                let mut d1 = start_date.day() as i64;
                let mut d2 = end_date.day() as i64;
                
                if d1 == 31 {
                    d1 = 30;
                }
                if d2 == 31 && d1 == 30 {
                    d2 = 30;
                }

                let y1 = start_date.year() as i64;
                let y2 = end_date.year() as i64;
                let m1 = start_date.month() as i64;
                let m2 = end_date.month() as i64;

                360 * (y2 - y1) + 30 * (m2 - m1) + (d2 - d1)
            }
            DayCountConvention::ActualActual 
            | DayCountConvention::Actual365 
            | DayCountConvention::Actual360 => {
                (end_date - start_date).num_days()
            }
        }
    }

    /// Calculates the year fraction between two dates based on the convention.
    pub fn year_fraction(&self, start_date: NaiveDate, end_date: NaiveDate) -> f64 {
        let days = self.day_count(start_date, end_date) as f64;
        match self {
            DayCountConvention::Thirty360 => days / 360.0,
            DayCountConvention::Actual365 => days / 365.0,
            DayCountConvention::Actual360 => days / 360.0,
            DayCountConvention::ActualActual => {
                // Calculate fraction by splitting across years to account for leap years properly
                let mut y = start_date.year();
                let mut total_fraction = 0.0;
                let mut current_start = start_date;

                while y < end_date.year() {
                    let next_year_start = NaiveDate::from_ymd_opt(y + 1, 1, 1)
                        .expect("Invalid date for next year");
                    let days_in_year = if is_leap_year(y) { 366.0 } else { 365.0 };
                    let days_in_period = (next_year_start - current_start).num_days() as f64;
                    total_fraction += days_in_period / days_in_year;
                    
                    current_start = next_year_start;
                    y += 1;
                }

                let days_in_year = if is_leap_year(y) { 366.0 } else { 365.0 };
                let days_in_period = (end_date - current_start).num_days() as f64;
                total_fraction += days_in_period / days_in_year;
                
                total_fraction
            }
        }
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}
