use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PositionId(pub Uuid);

impl PositionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for PositionId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for PositionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PortfolioId(pub Uuid);

impl PortfolioId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for PortfolioId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for PortfolioId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
