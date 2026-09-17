use alm_cashflows::{Cashflow, CashflowGenerator, CashflowSchedule};
use alm_core::NaiveDate;
use alm_time::business_day::BusinessDayConvention;
use alm_time::calendar::WeekendCalendar;
use alm_time::frequency::Frequency;
use alm_time::period::{Period, PeriodUnit};
use alm_time::schedule::ScheduleBuilder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixedRateBond {
    pub principal: f64,
    pub annual_coupon_rate: f64,
    pub start_date: NaiveDate,
    pub maturity_date: NaiveDate,
    pub frequency: Frequency,
}

impl FixedRateBond {
    pub fn new(
        principal: f64,
        annual_coupon_rate: f64,
        start_date: NaiveDate,
        maturity_date: NaiveDate,
        frequency: Frequency,
    ) -> Self {
        Self {
            principal,
            annual_coupon_rate,
            start_date,
            maturity_date,
            frequency,
        }
    }
}

impl CashflowGenerator for FixedRateBond {
    fn generate_cashflows(&self) -> CashflowSchedule {
        let mut cashflows = Vec::new();

        // Edge case: Bullet/Zero coupon
        if self.frequency == Frequency::Zero {
            cashflows.push(Cashflow::total(
                self.maturity_date,
                self.principal + (self.principal * self.annual_coupon_rate),
            ));
            return CashflowSchedule::new(cashflows);
        }

        // Determine tenor from frequency
        let months_per_period = self.frequency.months_per_period().unwrap_or(12);
        let tenor = Period::new(months_per_period as i32, PeriodUnit::Months);

        let calendar = WeekendCalendar;
        
        let schedule = ScheduleBuilder::new(
            self.start_date,
            self.maturity_date,
            tenor,
            BusinessDayConvention::NoAdjustment,
            &calendar,
        ).build();

        let dates = schedule.dates();
        
        // No cashflows if we just have the start date
        if dates.len() < 2 {
            return CashflowSchedule::new(cashflows);
        }

        // Standard coupon amount per period
        // For simplicity, assuming exactly equal periods rather than specific day count logic per coupon
        let coupon_amount = self.principal * (self.annual_coupon_rate / self.frequency.periods_per_year());

        // We skip index 0 because that is the start_date (no coupon paid at start)
        for i in 1..dates.len() {
            let payment_date = dates[i];
            
            // Final payment includes principal
            if i == dates.len() - 1 {
                cashflows.push(Cashflow::total(payment_date, self.principal + coupon_amount));
            } else {
                cashflows.push(Cashflow::interest(payment_date, coupon_amount));
            }
        }

        CashflowSchedule::new(cashflows)
    }
}

