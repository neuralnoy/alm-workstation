use alm_core::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Cashflow {
    pub date: NaiveDate,
    pub amount: f64,
}

impl Cashflow {
    pub fn new(date: NaiveDate, amount: f64) -> Self {
        Self { date, amount }
    }
}
