/// Represents the frequency of events (e.g., payments, compounding).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Frequency {
    Annual,
    SemiAnnual,
    Quarterly,
    Monthly,
    Weekly,
    Daily,
    Zero, // Represents a bullet/zero-coupon payment
    Continuous, // Continuous compounding
}

impl Frequency {
    /// Returns the number of periods in one year for a given frequency.
    /// Returns 0.0 for Zero frequency.
    pub fn periods_per_year(&self) -> f64 {
        match self {
            Frequency::Annual => 1.0,
            Frequency::SemiAnnual => 2.0,
            Frequency::Quarterly => 4.0,
            Frequency::Monthly => 12.0,
            Frequency::Weekly => 52.0,
            Frequency::Daily => 365.0, // Assuming standard 365-day base for frequency calculations
            Frequency::Zero => 0.0,
            Frequency::Continuous => f64::INFINITY,
        }
    }

    /// Number of months between periods.
    /// Returns `None` if the frequency is not evenly divisible into months.
    pub fn months_per_period(&self) -> Option<u32> {
        match self {
            Frequency::Annual => Some(12),
            Frequency::SemiAnnual => Some(6),
            Frequency::Quarterly => Some(3),
            Frequency::Monthly => Some(1),
            _ => None,
        }
    }
}
