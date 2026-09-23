pub fn interpolate_linear(x: f64, x1: f64, x2: f64, y1: f64, y2: f64) -> f64 {
    if (x2 - x1).abs() < 1e-10 {
        return y1;
    }
    let weight = (x - x1) / (x2 - x1);
    y1 + weight * (y2 - y1)
}

pub fn interpolate_log_linear(x: f64, x1: f64, x2: f64, y1: f64, y2: f64) -> f64 {
    if (x2 - x1).abs() < 1e-10 {
        return y1;
    }
    if y1 > 0.0 && y2 > 0.0 {
        let weight = (x - x1) / (x2 - x1);
        let ln_y = y1.ln() + weight * (y2.ln() - y1.ln());
        ln_y.exp()
    } else {
        interpolate_linear(x, x1, x2, y1, y2)
    }
}