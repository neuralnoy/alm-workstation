use crate::{CurveId, ZeroCurve};
use crate::interpolation::InterpolationMethod;
use alm_core::InterestRate;
use alm_time::day_count::DayCountConvention;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BootstrapInstrument {
    /// Cash deposit. Single payment at maturity.
    Cash {
        maturity: NaiveDate,
        rate: InterestRate,
    },
    /// Forward Rate Agreement.
    Fra {
        start: NaiveDate,
        end: NaiveDate,
        rate: InterestRate,
    },
    /// Interest Rate Swap (or OIS).
    Swap {
        payment_dates: Vec<NaiveDate>,
        rate: InterestRate,
    },
}

pub struct Bootstrapper {
    pub id: CurveId,
    pub as_of_date: NaiveDate,
    pub day_count: DayCountConvention,
    pub interpolation: InterpolationMethod,
    pub instruments: Vec<BootstrapInstrument>,
}

impl Bootstrapper {
    pub fn new(
        id: CurveId,
        as_of_date: NaiveDate,
        day_count: DayCountConvention,
        interpolation: InterpolationMethod,
    ) -> Self {
        Self {
            id,
            as_of_date,
            day_count,
            interpolation,
            instruments: Vec::new(),
        }
    }

    pub fn add_instrument(&mut self, instrument: BootstrapInstrument) {
        self.instruments.push(instrument);
    }

    pub fn build(self) -> ZeroCurve {
        // Start with the base node
        let mut x_points = vec![0.0];
        let mut y_points = vec![0.0]; // continuous zero rate at t=0
        let mut date_points = vec![self.as_of_date];

        for inst in &self.instruments {
            match inst {
                BootstrapInstrument::Cash { maturity, rate } => {
                    let t = self.day_count.year_fraction(self.as_of_date, *maturity);
                    let df = rate.discount_factor(t);
                    let r = if t > 0.0 { -df.ln() / t } else { 0.0 };
                    
                    x_points.push(t);
                    y_points.push(r);
                    date_points.push(*maturity);
                }
                BootstrapInstrument::Fra { start, end, rate } => {
                    let t_start = self.day_count.year_fraction(self.as_of_date, *start);
                    let t_end = self.day_count.year_fraction(self.as_of_date, *end);
                    let dt = self.day_count.year_fraction(*start, *end);

                    // Interpolate r at t_start
                    let r_start = self.interpolation.interpolate(t_start, &x_points, &y_points);
                    let df_start = (-r_start * t_start).exp();
                    
                    // Simple rate FRA formula: df_end = df_start / (1 + R * dt)
                    // Assuming simple compounding for FRA standard
                    let df_end = df_start / (1.0 + rate.value * dt);
                    let r_end = -df_end.ln() / t_end;

                    x_points.push(t_end);
                    y_points.push(r_end);
                    date_points.push(*end);
                }
                BootstrapInstrument::Swap { payment_dates, rate } => {
                    let end_date = *payment_dates.last().unwrap();
                    let t_end = self.day_count.year_fraction(self.as_of_date, end_date);
                    
                    // Root finding using Secant Method
                    let target_fn = |r_guess: f64| -> f64 {
                        let mut temp_x = x_points.clone();
                        let mut temp_y = y_points.clone();
                        temp_x.push(t_end);
                        temp_y.push(r_guess);

                        let mut npv = 0.0;
                        let mut prev_date = self.as_of_date;
                        for &pay_date in payment_dates {
                            let dt = self.day_count.year_fraction(prev_date, pay_date);
                            let t = self.day_count.year_fraction(self.as_of_date, pay_date);
                            let r = self.interpolation.interpolate(t, &temp_x, &temp_y);
                            let df = (-r * t).exp();
                            npv += rate.value * dt * df;
                            prev_date = pay_date;
                        }
                        
                        let r_end_eval = self.interpolation.interpolate(t_end, &temp_x, &temp_y);
                        let final_df = (-r_end_eval * t_end).exp();
                        npv + final_df - 1.0 // NPV of fixed leg + principal should equal par (1.0)
                    };

                    // Initial guesses
                    let r0 = *y_points.last().unwrap();
                    let r1 = r0 + 0.01;
                    
                    let solved_r = secant_solve(target_fn, r0, r1, 1e-8, 50);

                    x_points.push(t_end);
                    y_points.push(solved_r);
                    date_points.push(end_date);
                }
            }
        }

        // Convert back to inputs for ZeroCurve
        let mut final_inputs = Vec::with_capacity(date_points.len());
        for i in 1..date_points.len() {
            final_inputs.push((
                date_points[i], 
                InterestRate::new(y_points[i], alm_core::Compounding::Continuous, alm_time::frequency::Frequency::Continuous)
            ));
        }

        ZeroCurve::new(
            self.id,
            self.as_of_date,
            self.day_count,
            self.interpolation,
            final_inputs,
        )
    }
}

fn secant_solve<F: Fn(f64) -> f64>(f: F, mut x0: f64, mut x1: f64, tol: f64, max_iter: usize) -> f64 {
    for _ in 0..max_iter {
        let f0 = f(x0);
        let f1 = f(x1);
        if (f1 - f0).abs() < 1e-12 {
            break;
        }
        let x2 = x1 - f1 * (x1 - x0) / (f1 - f0);
        if (x2 - x1).abs() < tol {
            return x2;
        }
        x0 = x1;
        x1 = x2;
    }
    x1
}
