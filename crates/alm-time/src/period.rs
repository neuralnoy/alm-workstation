use chrono::{Datelike, Duration, NaiveDate};

/// Defines the unit of a period.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PeriodUnit {
    Days,
    Weeks,
    Months,
    Years,
}

/// Represents a period or tenor in financial calculations (e.g., 3 Months, 5 Years).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Period {
    pub length: i32,
    pub unit: PeriodUnit,
}

impl Period {
    pub fn new(length: i32, unit: PeriodUnit) -> Self {
        Self { length, unit }
    }

    pub fn days(length: i32) -> Self {
        Self::new(length, PeriodUnit::Days)
    }

    pub fn weeks(length: i32) -> Self {
        Self::new(length, PeriodUnit::Weeks)
    }

    pub fn months(length: i32) -> Self {
        Self::new(length, PeriodUnit::Months)
    }

    pub fn years(length: i32) -> Self {
        Self::new(length, PeriodUnit::Years)
    }

    /// Adds this period to a given date.
    /// Handles month-end adjustments (e.g., adding 1 month to Jan 31st gives Feb 28th/29th).
    pub fn add_to_date(&self, date: NaiveDate) -> NaiveDate {
        match self.unit {
            PeriodUnit::Days => date + Duration::days(self.length as i64),
            PeriodUnit::Weeks => date + Duration::days(self.length as i64 * 7),
            PeriodUnit::Months => add_months(date, self.length),
            PeriodUnit::Years => add_months(date, self.length * 12),
        }
    }
}

/// Helper function to add a given number of months to a date.
/// Preserves end-of-month logic, preventing overflow into the next month.
fn add_months(date: NaiveDate, months_to_add: i32) -> NaiveDate {
    let mut year = date.year();
    let mut month = date.month() as i32 + months_to_add;

    // Normalize the month and year
    while month > 12 {
        month -= 12;
        year += 1;
    }
    while month < 1 {
        month += 12;
        year -= 1;
    }

    let next_month = month as u32;
    
    // Determine the last valid day for the target month/year
    let last_day_of_target_month = match next_month {
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 31,
    };

    // If the original date's day exceeds the new month's length, truncate to the last valid day
    let next_day = std::cmp::min(date.day(), last_day_of_target_month);

    NaiveDate::from_ymd_opt(year, next_month, next_day)
        .expect("Invalid date calculated in add_months")
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}
