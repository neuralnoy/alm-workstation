use alm_curves::{
    bootstrap::{BootstrapInstrument, Bootstrapper},
    curve_set::CurveSet,
    shock::{CurveShift, CurveShock},
    CurveId, YieldCurve, YieldTermStructure,
};
use alm_math::interpolation::InterpolationMethod;
use alm_core::{Compounding, InterestRate};
use alm_time::{day_count::DayCountConvention, frequency::Frequency};
use chrono::NaiveDate;

fn main() {
    println!("=== alm-curves Example Usage ===");

    let as_of_date = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
    let day_count = DayCountConvention::Actual365;

    // 1. Bootstrapping a Zero Curve
    println!("\n1. Bootstrapping a new USD-OIS curve...");
    let mut bootstrapper = Bootstrapper::new(
        CurveId::new("USD-OIS"),
        as_of_date,
        day_count,
        InterpolationMethod::MonotoneConvex,
    );

    // Add 1M Cash deposit
    let d_1m = NaiveDate::from_ymd_opt(2023, 2, 1).unwrap();
    bootstrapper.add_instrument(BootstrapInstrument::Cash {
        maturity: d_1m,
        rate: InterestRate::new(0.045, Compounding::Simple, Frequency::Zero),
    });

    // Add 3M FRA (starting in 1M, ending in 4M)
    let d_4m = NaiveDate::from_ymd_opt(2023, 5, 1).unwrap();
    bootstrapper.add_instrument(BootstrapInstrument::Fra {
        start: d_1m,
        end: d_4m,
        rate: InterestRate::new(0.047, Compounding::Simple, Frequency::Zero),
    });

    // Add 1Y Swap (pays semi-annually)
    let d_6m = NaiveDate::from_ymd_opt(2023, 7, 1).unwrap();
    let d_1y = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    bootstrapper.add_instrument(BootstrapInstrument::Swap {
        payment_dates: vec![d_6m, d_1y],
        rate: InterestRate::new(0.050, Compounding::Simple, Frequency::SemiAnnual), 
    });

    // Build the zero curve
    let zero_curve = bootstrapper.build();
    println!("Successfully bootstrapped {} nodes.", zero_curve.points.len());

    // 2. Wrap it in a YieldCurve enum and add to CurveSet
    let mut curve_set = CurveSet::new();
    let yield_curve = YieldCurve::Zero(zero_curve);
    curve_set.add_curve(yield_curve.clone());
    println!("Added curve to CurveSet.");

    // 3. Querying rates
    println!("\n3. Querying rates from the curve:");
    let curve = curve_set.get_curve(&CurveId::new("USD-OIS")).unwrap();
    
    let target_date = NaiveDate::from_ymd_opt(2023, 8, 1).unwrap(); // Interpolated date
    
    let df = curve.discount_factor(target_date);
    println!("  Discount Factor at {}: {:.6}", target_date, df);
    
    let zero = curve.zero_rate(target_date, Compounding::Continuous, Frequency::Continuous);
    println!("  Continuous Zero Rate at {}: {:.4}%", target_date, zero.value * 100.0);

    let fwd_start = NaiveDate::from_ymd_opt(2023, 8, 1).unwrap();
    let fwd_end = NaiveDate::from_ymd_opt(2023, 11, 1).unwrap();
    let fwd = curve.forward_rate(fwd_start, fwd_end, Compounding::Simple, Frequency::Zero);
    println!("  Simple 3M Forward Rate (starting {}): {:.4}%", fwd_start, fwd.value * 100.0);

    // 4. Applying Risk Shocks
    println!("\n4. Applying Risk Shocks (DV01):");
    let up_shock = CurveShock::ParallelShift(0.0001); // +1 bps
    let shocked_curve = curve.as_ref().apply_shock(&up_shock);
    
    let shocked_df = shocked_curve.discount_factor(target_date);
    println!("  Shocked Discount Factor at {}: {:.6}", target_date, shocked_df);
    println!("  DV01 (change in DF for 1bp shift): {:.8}", df - shocked_df);
}
