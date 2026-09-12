use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum AlmError {
    #[error("Currency mismatch: expected {expected}, found {found}")]
    CurrencyMismatch { expected: String, found: String },

    #[error("Invalid interest rate: {0}")]
    InvalidRate(String),

    #[error("Invalid percentage: {0}")]
    InvalidPercentage(String),

    #[error("Invalid date operation: {0}")]
    DateError(String),

    #[error("Domain error: {0}")]
    DomainError(String),
}
