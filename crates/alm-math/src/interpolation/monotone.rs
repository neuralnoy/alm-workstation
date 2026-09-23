pub fn interpolate_monotone_convex(
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
    
    let x1 = x_points[idx];
    let x2 = x_points[idx + 1];
    let y1 = y_points[idx];
    let y2 = y_points[idx + 1];

    if (x2 - x1).abs() < 1e-10 {
        return y1;
    }

    // Function to calculate secant slope m_i
    let calc_m = |i: usize| -> f64 {
        (y_points[i + 1] - y_points[i]) / (x_points[i + 1] - x_points[i])
    };

    // Calculate the necessary m values
    let m_idx = calc_m(idx);
    let m_prev = if idx > 0 { calc_m(idx - 1) } else { m_idx };
    let m_next = if idx + 2 < n { calc_m(idx + 1) } else { m_idx };

    // Function to calculate tangent d_i (Fritsch-Carlson)
    let calc_d = |m_prev: f64, m_curr: f64| -> f64 {
        if m_prev * m_curr <= 0.0 {
            0.0
        } else {
            2.0 / (1.0 / m_prev + 1.0 / m_curr)
        }
    };

    let d1 = if idx == 0 { m_idx } else { calc_d(m_prev, m_idx) };
    let d2 = if idx + 2 == n { m_idx } else { calc_d(m_idx, m_next) };

    let h = x2 - x1;
    let t = (x - x1) / h;

    let h00 = 2.0 * t * t * t - 3.0 * t * t + 1.0;
    let h10 = t * t * t - 2.0 * t * t + t;
    let h01 = -2.0 * t * t * t + 3.0 * t * t;
    let h11 = t * t * t - t * t;

    h00 * y1 + h10 * h * d1 + h01 * y2 + h11 * h * d2
}