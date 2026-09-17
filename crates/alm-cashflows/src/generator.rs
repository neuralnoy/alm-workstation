use crate::schedule::CashflowSchedule;

/// A trait for any instrument or component that can generate a stream of cashflows.
pub trait CashflowGenerator {
    /// Generates the deterministic or expected cashflows.
    fn generate_cashflows(&self) -> CashflowSchedule;
}
