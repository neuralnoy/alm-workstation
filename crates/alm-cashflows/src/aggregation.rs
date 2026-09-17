use alm_core::NaiveDate;
use crate::cashflow::{Cashflow, CashflowType};
use crate::schedule::CashflowSchedule;
use std::collections::BTreeMap;

pub struct CashflowAggregator;

impl CashflowAggregator {
    /// Aggregates multiple cashflow schedules into a single schedule, summing amounts for the same date and type.
    pub fn aggregate(schedules: &[&CashflowSchedule]) -> CashflowSchedule {
        let mut map: BTreeMap<(NaiveDate, CashflowType), f64> = BTreeMap::new();

        for schedule in schedules {
            for cf in schedule.iter() {
                let key = (cf.date, cf.cashflow_type);
                *map.entry(key).or_insert(0.0) += cf.amount;
            }
        }

        let mut aggregated = Vec::new();
        for ((date, cashflow_type), amount) in map {
            aggregated.push(Cashflow::new(date, amount, cashflow_type));
        }

        CashflowSchedule::new(aggregated)
    }
}
