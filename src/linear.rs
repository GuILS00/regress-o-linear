pub fn linear_regression(x: &[f64], y: &[f64]) -> Result<(f64, f64), &'static str> {
    if x.len() != y.len() || x.is_empty() {
        return Err("Vetores inválidos para regressão.");
    }

    let n = x.len() as f64;
    let soma_x: f64 = x.iter().sum();
    let soma_y: f64 = y.iter().sum();
    let soma_xy: f64 = x.iter().zip(y.iter()).map(|(xi, yi)| xi * yi).sum();
    let soma_x2: f64 = x.iter().map(|xi| xi * xi).sum();

    let divisor = n * soma_x2 - soma_x * soma_x;
    if divisor == 0.0 {
        return Err("Divisão por zero na regressão.");
    }

    let a = (n * soma_xy - soma_x * soma_y) / divisor;
    let b = (soma_y - a * soma_x) / n;

    Ok((a, b))
}

pub fn prever_valores(x: &[f64], a: f64, b: f64) -> Vec<f64> {
    x.iter().map(|xi| a * xi + b).collect()
}

pub fn calcular_r2(x: &[f64], y: &[f64], a: f64, b: f64) -> f64 {
    let y_medio: f64 = y.iter().sum::<f64>() / y.len() as f64;
    let ss_total: f64 = y.iter().map(|yi| (yi - y_medio).powi(2)).sum();
    let ss_res: f64 = x.iter().zip(y.iter())
        .map(|(xi, yi)| (yi - (a * xi + b)).powi(2))
        .sum();
    1.0 - ss_res / ss_total
}

pub fn calcular_mse(x: &[f64], y: &[f64], a: f64, b: f64) -> f64 {
    x.iter().zip(y.iter())
        .map(|(xi, yi)| (yi - (a * xi + b)).powi(2))
        .sum::<f64>() / x.len() as f64
}
