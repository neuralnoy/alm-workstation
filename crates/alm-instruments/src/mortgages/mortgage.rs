use alm_cashflows::{Cashflow, CashflowGenerator, CashflowSchedule};
use alm_core::NaiveDate;
use alm_time::business_day::BusinessDayConvention;
use alm_time::calendar::WeekendCalendar;
use alm_time::frequency::Frequency;
use alm_time::period::{Period, PeriodUnit};
use alm_time::schedule::ScheduleBuilder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mortgage {
    pub principal: f64,
    pub annual_interest_rate: f64,
    pub start_date: NaiveDate,
    pub maturity_date: NaiveDate,
    pub frequency: Frequency,
}

impl Mortgage {
    pub fn new(
        principal: f64,
        annual_interest_rate: f64,
        start_date: NaiveDate,
        maturity_date: NaiveDate,
        frequency: Frequency,
    ) -> Self {
        Self {
            principal,
            annual_interest_rate,
            start_date,
            maturity_date,
            frequency,
        }
    }
}

impl CashflowGenerator for Mortgage {
    fn generate_cashflows(&self) -> CashflowSchedule {
        let mut cashflows = Vec::new();

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
        
        if dates.len() < 2 {
            return CashflowSchedule::new(cashflows);
        }

        let num_payments = (dates.len() - 1) as f64;
        let rate_per_period = self.annual_interest_rate / self.frequency.periods_per_year();
        
        // Annuity payment formula: P = (r * PV) / (1 - (1 + r)^-n)
        let payment_amount = if rate_per_period > 0.0 {
            (rate_per_period * self.principal) / (1.0 - (1.0 + rate_per_period).powf(-num_payments))
        } else {
            self.principal / num_payments
        };

        let mut remaining_principal = self.principal;

        for i in 1..dates.len() {
            let payment_date = dates[i];
            
            let interest_payment = remaining_principal * rate_per_period;
            let principal_payment = if i == dates.len() - 1 {
                // Last payment pays off whatever is left due to rounding
                remaining_principal
            } else {
                payment_amount - interest_payment
            };

            cashflows.push(Cashflow::interest(payment_date, interest_payment));
            cashflows.push(Cashflow::principal(payment_date, principal_payment));
            
            remaining_principal -= principal_payment;
        }

        CashflowSchedule::new(cashflows)
    }
}
