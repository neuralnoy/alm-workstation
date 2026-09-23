use alm_cashflows::{Cashflow, CashflowGenerator};
use alm_core::{Compounding, InterestRate, NaiveDate};
use alm_curves::{
    bootstrap::{BootstrapInstrument, Bootstrapper},
    CurveId, YieldTermStructure, ZeroCurve,
};
use alm_math::interpolation::InterpolationMethod;
use alm_instruments::{fixed_income::FixedRateBond, mortgages::Mortgage};
use alm_risk::{calculate_macaulay_duration, calculate_pv};
use alm_time::day_count::DayCountConvention;
use alm_time::frequency::Frequency;
use chrono::Datelike;
use serde::Serialize;

#[derive(Serialize)]
pub struct CurvePointResponse {
    pub date: String,
    pub time_years: f64,
    pub zero_rate: f64,
    pub forward_rate: f64,
    pub discount_factor: f64,
}

#[tauri::command]
fn get_curve_data() -> Result<Vec<CurvePointResponse>, String> {
    let as_of_date = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
    let day_count = DayCountConvention::Actual365;

    let mut bootstrapper = Bootstrapper::new(
        CurveId::new("USD-OIS"),
        as_of_date,
        day_count,
        InterpolationMethod::MonotoneConvex,
    );

    let d_1m = NaiveDate::from_ymd_opt(2023, 2, 1).unwrap();
    bootstrapper.add_instrument(BootstrapInstrument::Cash {
        maturity: d_1m,
        rate: InterestRate::new(0.045, Compounding::Simple, Frequency::Zero),
    });

    let d_4m = NaiveDate::from_ymd_opt(2023, 5, 1).unwrap();
    bootstrapper.add_instrument(BootstrapInstrument::Fra {
        start: d_1m,
        end: d_4m,
        rate: InterestRate::new(0.047, Compounding::Simple, Frequency::Zero),
    });

    let d_6m = NaiveDate::from_ymd_opt(2023, 7, 1).unwrap();
    let d_1y = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    bootstrapper.add_instrument(BootstrapInstrument::Swap {
        payment_dates: vec![d_6m, d_1y],
        rate: InterestRate::new(0.050, Compounding::Simple, Frequency::SemiAnnual), 
    });

    // We add a few more swaps to get a nice 10-year curve!
    let d_2y = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
    bootstrapper.add_instrument(BootstrapInstrument::Swap {
        payment_dates: vec![d_6m, d_1y, NaiveDate::from_ymd_opt(2024, 7, 1).unwrap(), d_2y],
        rate: InterestRate::new(0.052, Compounding::Simple, Frequency::SemiAnnual), 
    });
    let d_5y = NaiveDate::from_ymd_opt(2028, 1, 1).unwrap();
    bootstrapper.add_instrument(BootstrapInstrument::Swap {
        payment_dates: vec![
            d_6m, d_1y, NaiveDate::from_ymd_opt(2024, 7, 1).unwrap(), d_2y,
            NaiveDate::from_ymd_opt(2025, 7, 1).unwrap(), NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(), NaiveDate::from_ymd_opt(2027, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2027, 7, 1).unwrap(), d_5y
        ],
        rate: InterestRate::new(0.055, Compounding::Simple, Frequency::SemiAnnual), 
    });
    let d_10y = NaiveDate::from_ymd_opt(2033, 1, 1).unwrap();
    let mut pay_dates_10y = Vec::new();
    let mut curr = NaiveDate::from_ymd_opt(2023, 7, 1).unwrap();
    while curr <= d_10y {
        pay_dates_10y.push(curr);
        let next_month = curr.month() + 6;
        let y = if next_month > 12 { curr.year() + 1 } else { curr.year() };
        let m = if next_month > 12 { next_month - 12 } else { next_month };
        curr = NaiveDate::from_ymd_opt(y, m, 1).unwrap();
    }
    bootstrapper.add_instrument(BootstrapInstrument::Swap {
        payment_dates: pay_dates_10y,
        rate: InterestRate::new(0.058, Compounding::Simple, Frequency::SemiAnnual), 
    });

    let curve = bootstrapper.build();
    let mut response = Vec::new();

    let mut current_date = as_of_date;
    for _ in 0..120 {
        let next_month = current_date.month() + 1;
        let mut year = current_date.year();
        let mut month = next_month;
        if month > 12 {
            month = 1;
            year += 1;
        }
        current_date = NaiveDate::from_ymd_opt(year, month, current_date.day()).unwrap_or(
            NaiveDate::from_ymd_opt(year, month, 28).unwrap()
        );
        
        let t = curve.day_count().year_fraction(as_of_date, current_date);
        let df = curve.discount_factor(current_date);
        let zero = curve.zero_rate(current_date, Compounding::Continuous, Frequency::Continuous).value;
        
        let fwd_end = current_date + chrono::Duration::days(90);
        let fwd = curve.forward_rate(current_date, fwd_end, Compounding::Simple, Frequency::Zero).value;

        response.push(CurvePointResponse {
            date: current_date.to_string(),
            time_years: t,
            zero_rate: zero,
            forward_rate: fwd,
            discount_factor: df,
        });
    }

    Ok(response)
}

#[derive(Serialize)]
pub struct BondMetricsResponse {
    pub cashflows: Vec<Cashflow>,
    pub present_value: f64,
    pub macaulay_duration: f64,
    pub modified_duration: f64,
}

#[tauri::command]
fn calculate_instrument_metrics(
    instrument_type: &str,
    principal: f64,
    coupon_rate: f64,
    start_date: &str,
    maturity_date: &str,
    discount_rate: f64,
) -> Result<BondMetricsResponse, String> {
    let start_date = NaiveDate::parse_from_str(start_date, "%Y-%m-%d").map_err(|e| e.to_string())?;
    let maturity_date =
        NaiveDate::parse_from_str(maturity_date, "%Y-%m-%d").map_err(|e| e.to_string())?;

    let cashflows = if instrument_type == "Mortgage" {
        let mortgage = Mortgage::new(
            principal,
            coupon_rate,
            start_date,
            maturity_date,
            Frequency::Monthly,
        );
        mortgage.generate_cashflows()
    } else {
        let bond = FixedRateBond::new(
            principal,
            coupon_rate,
            start_date,
            maturity_date,
            Frequency::SemiAnnual,
        );
        bond.generate_cashflows()
    };

    // Create a flat zero curve for discounting
    let as_of_date = start_date;
    let rate = InterestRate::new(discount_rate, Compounding::Continuous, Frequency::Continuous);
    
    // We just need a single point in the future for a flat curve
    let far_future = NaiveDate::from_ymd_opt(2100, 1, 1).unwrap();
    let curve = ZeroCurve::new(
        CurveId::new("DISCOUNT"),
        as_of_date,
        DayCountConvention::Actual365,
        InterpolationMethod::Linear,
        vec![(as_of_date, rate), (far_future, rate)],
    );

    // Calculate metrics
    let pv = calculate_pv(&cashflows, &curve);
    let mac_dur = calculate_macaulay_duration(&cashflows, &curve);
    
    // Modified Duration = Macaulay Duration / (1 + y/n)
    // Assuming continuous compounding for curve, modified duration is just macaulay duration
    // If it was semi-annual compounding, it would be mac_dur / (1.0 + discount_rate/2.0)
    let mod_dur = mac_dur;

    Ok(BondMetricsResponse {
        cashflows: cashflows.cashflows.clone(),
        present_value: pv,
        macaulay_duration: mac_dur,
        modified_duration: mod_dur,
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![calculate_instrument_metrics, get_curve_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
