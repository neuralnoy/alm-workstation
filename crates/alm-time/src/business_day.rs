use chrono::{Datelike, Duration, NaiveDate};
use crate::calendar::Calendar;
use serde::{Deserialize, Serialize};

/// Defines how days in an accrual period are adjusted when they fall on non-business days.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BusinessDayConvention {
    /// The timing of cash flows is not adjusted for non-business days.
    NoAdjustment,
    /// If a payment date falls on a non-business day, it is adjusted to the preceding business day.
    Previous,
    /// If a payment date falls on a non-business day, it is adjusted to the preceding business day,
    /// unless this causes the payment date to fall into the previous calendar month, in which case
    /// the next business day is used.
    ModifiedPrevious,
    /// If a payment date falls on a non-business day, it is adjusted to the next business day.
    Following,
    /// If a payment date falls on a non-business day, it is adjusted to the next business day,
    /// unless this causes the cash flow to fall into the next calendar month, in which case
    /// the business day before that non-business day is used.
    ModifiedFollowing,
    /// The cash flow date is set to occur at the end of the month even if it is a non-business day.
    EndOfMonthNoAdjustment,
}

impl BusinessDayConvention {
    /// Adjusts the given date according to the business day convention and the provided calendar.
    pub fn adjust<C: Calendar>(&self, date: NaiveDate, calendar: &C) -> NaiveDate {
        match self {
            BusinessDayConvention::NoAdjustment => date,
            BusinessDayConvention::Previous => {
                let mut adjusted = date;
                while !calendar.is_business_day(adjusted) {
                    adjusted -= Duration::days(1);
                }
                adjusted
            }
            BusinessDayConvention::ModifiedPrevious => {
                let mut adjusted = date;
                while !calendar.is_business_day(adjusted) {
                    adjusted -= Duration::days(1);
                }
                if adjusted.month() != date.month() {
                    // Go forward instead
                    adjusted = date;
                    while !calendar.is_business_day(adjusted) {
                        adjusted += Duration::days(1);
                    }
                }
                adjusted
            }
            BusinessDayConvention::Following => {
                let mut adjusted = date;
                while !calendar.is_business_day(adjusted) {
                    adjusted += Duration::days(1);
                }
                adjusted
            }
            BusinessDayConvention::ModifiedFollowing => {
                let mut adjusted = date;
                while !calendar.is_business_day(adjusted) {
                    adjusted += Duration::days(1);
                }
                if adjusted.month() != date.month() {
                    // Go backward instead
                    adjusted = date;
                    while !calendar.is_business_day(adjusted) {
                        adjusted -= Duration::days(1);
                    }
                }
                adjusted
            }
            BusinessDayConvention::EndOfMonthNoAdjustment => {
                // Find the last day of the month
                let y = date.year();
                let m = date.month();
                let next_m = if m == 12 { 1 } else { m + 1 };
                let next_y = if m == 12 { y + 1 } else { y };
                let next_month_start = NaiveDate::from_ymd_opt(next_y, next_m, 1)
                    .expect("Invalid date for next month");
                next_month_start - Duration::days(1)
            }
        }
    }
}
