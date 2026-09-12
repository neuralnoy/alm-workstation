use chrono::{Datelike, Weekday, NaiveDate};

/// A trait for defining business day calendars.
pub trait Calendar {
    /// Returns true if the given date is a business day.
    fn is_business_day(&self, date: NaiveDate) -> bool;

    /// Returns true if the given date is a holiday or a weekend.
    fn is_holiday(&self, date: NaiveDate) -> bool {
        !self.is_business_day(date)
    }
}

/// A simple calendar that only considers weekends as non-business days.
#[derive(Debug, Clone, Copy, Default)]
pub struct WeekendCalendar;

impl Calendar for WeekendCalendar {
    fn is_business_day(&self, date: NaiveDate) -> bool {
        let weekday = date.weekday();
        weekday != Weekday::Sat && weekday != Weekday::Sun
    }
}
