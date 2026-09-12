use alm_time::business_day::BusinessDayConvention;
use alm_time::calendar::WeekendCalendar;
use alm_time::day_count::DayCountConvention;
use alm_time::frequency::Frequency;
use alm_time::period::Period;
use alm_time::schedule::ScheduleBuilder;
use chrono::NaiveDate;

fn main() {
    // 1. Setup your instrument parameters
    let start_date = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
    let maturity_date = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();

    // E.g., A loan paying quarterly
    let payment_tenor = Period::months(3);
    let _payment_frequency = Frequency::Quarterly;

    // 2. Setup your conventions
    let calendar = WeekendCalendar;
    let bdc = BusinessDayConvention::ModifiedFollowing;
    let dcc = DayCountConvention::Actual360;

    // 3. Generate the payment schedule
    // This generates dates by adding 3 months iteratively, then adjusting them
    // so they never fall on a weekend.
    let schedule =
        ScheduleBuilder::new(start_date, maturity_date, payment_tenor, bdc, &calendar).build();

    println!("Payment Schedule:");
    for date in schedule.dates() {
        println!("- {}", date);
    }

    // 4. Calculate interest accrual (Year Fraction)
    // Between any two dates, you use the DayCountConvention to figure out
    // what fraction of the annual interest rate to apply.
    let first_payment_date = schedule.dates()[1]; // assuming index 0 is start_date
    let accrual_fraction = dcc.year_fraction(start_date, first_payment_date);

    println!(
        "\nAccrual fraction for first period ({} to {}): {:.4}",
        start_date, first_payment_date, accrual_fraction
    );

    // If the annual rate is 5% and principal is $1000:
    let interest = 1000.0 * 0.05 * accrual_fraction;
    println!("Interest Payment: ${:.2}", interest);
}
