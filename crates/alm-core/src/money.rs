use super::currency::Currency;
use super::errors::AlmError;
use super::result::Result;
use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Money {
    pub amount: f64,
    pub currency: Currency,
}

impl Money {
    pub fn new(amount: f64, currency: Currency) -> Self {
        Self { amount, currency }
    }

    pub fn zero(currency: Currency) -> Self {
        Self {
            amount: 0.0,
            currency,
        }
    }

    pub fn is_zero(&self) -> bool {
        self.amount == 0.0
    }

    /// Safely adds another Money object, returning an error if currencies don't match.
    pub fn try_add(self, rhs: Self) -> Result<Self> {
        if self.currency != rhs.currency {
            return Err(AlmError::CurrencyMismatch {
                expected: self.currency.to_string(),
                found: rhs.currency.to_string(),
            });
        }
        Ok(Self::new(self.amount + rhs.amount, self.currency))
    }

    /// Safely subtracts another Money object, returning an error if currencies don't match.
    pub fn try_sub(self, rhs: Self) -> Result<Self> {
        if self.currency != rhs.currency {
            return Err(AlmError::CurrencyMismatch {
                expected: self.currency.to_string(),
                found: rhs.currency.to_string(),
            });
        }
        Ok(Self::new(self.amount - rhs.amount, self.currency))
    }
}

// Implement standard Add/Sub that panic on mismatch (useful for known-good operations)
impl Add for Money {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.try_add(rhs).expect("Currencies must match for addition")
    }
}

impl Sub for Money {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.try_sub(rhs).expect("Currencies must match for subtraction")
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2} {}", self.amount, self.currency)
    }
}

