pub fn interpolate_cubic_spline(
    x: f64,
    x_points: &[f64],
    y_points: &[f64],
    idx: usize,
) -> f64 {
    let n = x_points.len();
    if n < 2 {
        return y_points.first().copied().unwrap_or(0.0);
    }
    if idx >= n - 1 {
        return y_points[n - 1];
    }

    let mut h = vec![0.0; n - 1];
    for i in 0..n - 1 {
        let mut diff = x_points[i + 1] - x_points[i];
        if diff <= 0.0 {
            diff = 1e-10;
        }
        h[i] = diff;
    }

    let mut v = vec![0.0; n - 1];
    for i in 1..n - 1 {
        v[i] = 6.0 * ((y_points[i + 1] - y_points[i]) / h[i] - (y_points[i] - y_points[i - 1]) / h[i - 1]);
    }

    let mut c_prime = vec![0.0; n];
    let mut d_prime = vec![0.0; n];

    for i in 1..n - 1 {
        let a = h[i - 1];
        let b = 2.0 * (h[i - 1] + h[i]);
        let c = h[i];

        let denom = b - a * c_prime[i - 1];
        c_prime[i] = c / denom;
        d_prime[i] = (v[i] - a * d_prime[i - 1]) / denom;
    }

    let mut m = vec![0.0; n];
    m[n - 1] = 0.0;
    for i in (1..n - 1).rev() {
        m[i] = d_prime[i] - c_prime[i] * m[i + 1];
    }
    m[0] = 0.0;

    let h_i = h[idx];
    let a = (x_points[idx + 1] - x) / h_i;
    let b = (x - x_points[idx]) / h_i;

    a * y_points[idx]
        + b * y_points[idx + 1]
        + ((a * a * a - a) * m[idx] + (b * b * b - b) * m[idx + 1]) * (h_i * h_i) / 6.0
}