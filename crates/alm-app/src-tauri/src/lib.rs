use alm_cashflows::Cashflow;
use alm_core::{Compounding, InterestRate, NaiveDate};
use alm_curves::ZeroCurve;
use alm_instruments::fixed_income::FixedRateBond;
use alm_risk::{calculate_macaulay_duration, calculate_pv};
use alm_time::day_count::DayCountConvention;
use alm_time::frequency::Frequency;
use serde::Serialize;

#[derive(Serialize)]
pub struct BondMetricsResponse {
    pub cashflows: Vec<Cashflow>,
    pub present_value: f64,
    pub macaulay_duration: f64,
    pub modified_duration: f64,
}

#[tauri::command]
fn calculate_bond_metrics(
    principal: f64,
    coupon_rate: f64,
    start_date: &str,
    maturity_date: &str,
    discount_rate: f64,
) -> Result<BondMetricsResponse, String> {
    let start_date = NaiveDate::parse_from_str(start_date, "%Y-%m-%d").map_err(|e| e.to_string())?;
    let maturity_date =
        NaiveDate::parse_from_str(maturity_date, "%Y-%m-%d").map_err(|e| e.to_string())?;

    // Create the bond
    let bond = FixedRateBond::new(
        principal,
        coupon_rate,
        start_date,
        maturity_date,
        Frequency::SemiAnnual,
    );

    // Generate Cashflows
    let cashflows = bond.generate_cashflows();

    // Create a flat zero curve for discounting
    let as_of_date = start_date;
    let rate = InterestRate::new(discount_rate, Compounding::Continuous);
    
    // We just need a single point in the future for a flat curve
    let far_future = NaiveDate::from_ymd_opt(2100, 1, 1).unwrap();
    let curve = ZeroCurve::new(
        as_of_date,
        DayCountConvention::Actual365,
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
        cashflows,
        present_value: pv,
        macaulay_duration: mac_dur,
        modified_duration: mod_dur,
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![calculate_bond_metrics])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
