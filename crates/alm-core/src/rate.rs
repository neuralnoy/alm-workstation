use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Compounding {
    Simple,
    Compounded,
    Continuous,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct InterestRate {
    pub value: f64,
    pub compounding: Compounding,
}

impl InterestRate {
    pub fn new(value: f64, compounding: Compounding) -> Self {
        Self { value, compounding }
    }

    /// Calculate the discount factor for a given time fraction
    pub fn discount_factor(&self, time: f64) -> f64 {
        match self.compounding {
            Compounding::Simple => 1.0 / (1.0 + self.value * time),
            Compounding::Compounded => 1.0 / (1.0 + self.value).powf(time),
            Compounding::Continuous => (-self.value * time).exp(),
        }
    }

    /// Calculate the compound factor for a given time fraction
    pub fn compound_factor(&self, time: f64) -> f64 {
        1.0 / self.discount_factor(time)
    }
}
