use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Currency {
    USD,
    EUR,
    CHF,
    GBP,
    JPY,
    CAD,
    AUD,
    // Add more as needed
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for Currency {
    type Err = super::errors::AlmError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "USD" => Ok(Currency::USD),
            "EUR" => Ok(Currency::EUR),
            "CHF" => Ok(Currency::CHF),
            "GBP" => Ok(Currency::GBP),
            "JPY" => Ok(Currency::JPY),
            "CAD" => Ok(Currency::CAD),
            "AUD" => Ok(Currency::AUD),
            _ => Err(super::errors::AlmError::DomainError(format!(
                "Unsupported currency: {}",
                s
            ))),
        }
    }
}
