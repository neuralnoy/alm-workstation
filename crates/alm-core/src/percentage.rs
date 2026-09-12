use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Percentage(pub f64);

impl Percentage {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    /// Creates a percentage from a whole number (e.g., 5.0 -> 0.05)
    pub fn from_whole(value: f64) -> Self {
        Self(value / 100.0)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl Default for Percentage {
    fn default() -> Self {
        Self(0.0)
    }
}

impl Add for Percentage {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Percentage {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul<f64> for Percentage {
    type Output = f64;
    fn mul(self, rhs: f64) -> Self::Output {
        self.0 * rhs
    }
}

impl Div<f64> for Percentage {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl fmt::Display for Percentage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}%", self.0 * 100.0)
    }
}
