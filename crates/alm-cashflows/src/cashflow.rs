use alm_core::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub enum CashflowType {
    Principal,
    Interest,
    Fee,
    Total,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Cashflow {
    pub date: NaiveDate,
    pub amount: f64,
    pub cashflow_type: CashflowType,
}

impl Cashflow {
    pub fn new(date: NaiveDate, amount: f64, cashflow_type: CashflowType) -> Self {
        Self {
            date,
            amount,
            cashflow_type,
        }
    }

    pub fn principal(date: NaiveDate, amount: f64) -> Self {
        Self::new(date, amount, CashflowType::Principal)
    }

    pub fn interest(date: NaiveDate, amount: f64) -> Self {
        Self::new(date, amount, CashflowType::Interest)
    }

    pub fn fee(date: NaiveDate, amount: f64) -> Self {
        Self::new(date, amount, CashflowType::Fee)
    }

    pub fn total(date: NaiveDate, amount: f64) -> Self {
        Self::new(date, amount, CashflowType::Total)
    }
}
