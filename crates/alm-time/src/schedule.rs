use chrono::NaiveDate;
use crate::business_day::BusinessDayConvention;
use crate::calendar::Calendar;
use crate::period::Period;

/// Represents a generated schedule of dates for a financial instrument.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Schedule {
    dates: Vec<NaiveDate>,
}

impl Schedule {
    pub fn new(dates: Vec<NaiveDate>) -> Self {
        Self { dates }
    }

    pub fn dates(&self) -> &[NaiveDate] {
        &self.dates
    }
}

/// Builder for generating a standard financial schedule.
pub struct ScheduleBuilder<'a, C: Calendar> {
    start_date: NaiveDate,
    end_date: NaiveDate,
    tenor: Period,
    business_day_convention: BusinessDayConvention,
    calendar: &'a C,
}

impl<'a, C: Calendar> ScheduleBuilder<'a, C> {
    pub fn new(
        start_date: NaiveDate,
        end_date: NaiveDate,
        tenor: Period,
        business_day_convention: BusinessDayConvention,
        calendar: &'a C,
    ) -> Self {
        Self {
            start_date,
            end_date,
            tenor,
            business_day_convention,
            calendar,
        }
    }

    /// Generates the schedule of adjusted dates.
    pub fn build(self) -> Schedule {
        let mut unadjusted_dates = Vec::new();
        let mut current_unadjusted = self.start_date;

        unadjusted_dates.push(current_unadjusted);

        // Generate unadjusted dates based on the tenor
        // This is a simplified forward-generation schedule.
        // A robust implementation would also support backward-generation from the maturity date.
        let mut count = 1;
        while current_unadjusted < self.end_date {
            let step = Period::new(self.tenor.length * count, self.tenor.unit);
            let next_unadjusted = step.add_to_date(self.start_date);
            
            if next_unadjusted >= self.end_date {
                break;
            }
            unadjusted_dates.push(next_unadjusted);
            current_unadjusted = next_unadjusted;
            count += 1;
        }
        
        // Ensure the end date is included
        if unadjusted_dates.last() != Some(&self.end_date) {
            unadjusted_dates.push(self.end_date);
        }

        // Adjust dates according to business day convention
        let mut adjusted_dates = Vec::with_capacity(unadjusted_dates.len());
        for unadjusted in unadjusted_dates {
            let adjusted = self.business_day_convention.adjust(unadjusted, self.calendar);
            adjusted_dates.push(adjusted);
        }
        
        // Remove duplicate dates that can occur when multiple non-business days roll to the same business day
        adjusted_dates.dedup();

        Schedule::new(adjusted_dates)
    }
}
