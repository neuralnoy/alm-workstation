use alm_time::frequency::Frequency;
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
    pub frequency: Frequency,
}

impl InterestRate {
    pub fn new(value: f64, compounding: Compounding, frequency: Frequency) -> Self {
        Self {
            value,
            compounding,
            frequency,
        }
    }

    /// Calculate the discount factor for a given time fraction
    pub fn discount_factor(&self, time: f64) -> f64 {
        1.0 / self.compound_factor(time)
    }

    /// Calculate the compound factor for a given time fraction
    pub fn compound_factor(&self, time: f64) -> f64 {
        match self.compounding {
            Compounding::Simple => 1.0 + self.value * time,
            Compounding::Compounded => {
                let m = self.frequency.periods_per_year();
                if m.is_infinite() {
                    (self.value * time).exp()
                } else if m > 0.0 {
                    (1.0 + self.value / m).powf(m * time)
                } else {
                    (1.0 + self.value).powf(time)
                }
            }
            Compounding::Continuous => (self.value * time).exp(),
        }
    }
}
