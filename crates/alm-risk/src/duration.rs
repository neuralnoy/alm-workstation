use alm_cashflows::Cashflow;
use alm_curves::{ZeroCurve, YieldTermStructure};
use alm_time::day_count::DayCountConvention;

/// Calculates the Macaulay Duration of a series of cashflows given a discount curve.
pub fn calculate_macaulay_duration(cashflows: &[Cashflow], curve: &ZeroCurve) -> f64 {
    let mut total_pv = 0.0;
    let mut weighted_time = 0.0;
    
    // Use Actual/365 for simple time calculations here
    let day_count = DayCountConvention::Actual365;

    for cf in cashflows {
        let df = curve.discount_factor(cf.date);
        let pv_cf = cf.amount * df;
        total_pv += pv_cf;
        
        // Time in years from curve as_of_date to cashflow date
        let t = day_count.year_fraction(curve.as_of_date, cf.date);
        weighted_time += pv_cf * t;
    }

    if total_pv == 0.0 {
        return 0.0;
    }

    weighted_time / total_pv
}
