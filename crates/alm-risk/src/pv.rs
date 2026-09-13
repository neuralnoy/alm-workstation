use alm_cashflows::Cashflow;
use alm_curves::{ZeroCurve, YieldTermStructure};

/// Calculates the present value of a series of cashflows given a discount curve.
pub fn calculate_pv(cashflows: &[Cashflow], curve: &ZeroCurve) -> f64 {
    cashflows
        .iter()
        .map(|cf| {
            let df = curve.discount_factor(cf.date);
            cf.amount * df
        })
        .sum()
}
