use crate::cashflow::Cashflow;
use alm_core::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CashflowSchedule {
    pub cashflows: Vec<Cashflow>,
}

impl CashflowSchedule {
    pub fn new(mut cashflows: Vec<Cashflow>) -> Self {
        cashflows.sort_by_key(|c| c.date);
        Self { cashflows }
    }

    pub fn empty() -> Self {
        Self { cashflows: Vec::new() }
    }

    pub fn add_cashflow(&mut self, cashflow: Cashflow) {
        self.cashflows.push(cashflow);
        self.cashflows.sort_by_key(|c| c.date);
    }

    pub fn filter_by_date_range(&self, start: NaiveDate, end: NaiveDate) -> Self {
        let filtered = self
            .cashflows
            .iter()
            .filter(|c| c.date >= start && c.date <= end)
            .cloned()
            .collect();
        Self::new(filtered)
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Cashflow> {
        self.cashflows.iter()
    }
}

impl IntoIterator for CashflowSchedule {
    type Item = Cashflow;
    type IntoIter = std::vec::IntoIter<Cashflow>;

    fn into_iter(self) -> Self::IntoIter {
        self.cashflows.into_iter()
    }
}

impl std::ops::Deref for CashflowSchedule {
    type Target = [Cashflow];

    fn deref(&self) -> &Self::Target {
        &self.cashflows
    }
}
