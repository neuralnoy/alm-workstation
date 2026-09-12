use alm_core::{Currency, Money, Compounding, InterestRate, Percentage};
use rust_decimal_macros::dec;

fn main() {
    println!("--- ALM Core Foundation Examples ---\n");

    // 1. Working with Money and Currencies
    let principal = Money::new(dec!(10000.00), Currency::USD);
    let fee = Money::new(dec!(250.50), Currency::USD);
    
    // You can safely add money of the same currency
    let total_cost = principal.try_add(fee).expect("Currencies match");
    println!("Total Cost: {}", total_cost);

    // If you try to subtract a different currency, it returns an error
    let foreign_fee = Money::new(dec!(15.00), Currency::EUR);
    match total_cost.try_sub(foreign_fee) {
        Ok(new_total) => println!("New Total: {}", new_total),
        Err(e) => println!("Error prevented invalid arithmetic: {}", e),
    }

    println!();

    // 2. Working with Percentages
    let target_margin = Percentage::new(0.045); // 4.5%
    let extra_margin = Percentage::new(0.005);  // 0.5%
    let total_margin = target_margin + extra_margin;
    
    println!("Target Margin: {}", target_margin);
    println!("Total Margin with bump: {}", total_margin);
    println!();

    // 3. Working with Interest Rates
    let rate = InterestRate::new(0.05, Compounding::Continuous); // 5% continuous
    let time_horizon = 2.5; // 2.5 years

    let discount = rate.discount_factor(time_horizon);
    let compound = rate.compound_factor(time_horizon);

    println!("Rate: 5% Continuous");
    println!("Discount factor over 2.5 years: {:.4}", discount);
    println!("Compound factor over 2.5 years: {:.4}", compound);
    
    // Applying the factor to a decimal amount
    let future_value = principal.amount * rust_decimal::Decimal::from_f64_retain(compound).unwrap();
    let future_money = Money::new(future_value, Currency::USD);
    println!("Future Value of {} after 2.5 years: {}", principal, future_money);
}
