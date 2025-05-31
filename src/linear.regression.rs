/// Implementações para regressão linear simples e métricas estatísticas.

/// Realiza a regressão linear simples sobre uma série de valores.
/// Retorna o coeficiente angular (slope) e o coeficiente linear (intercept).
pub fn linear_regression(series: &[f64]) -> Result<(f64, f64), &'static str> {
    let n = series.len();
    if n < 2 {
        return Err("A série precisa ter ao menos dois valores.");
    }

    let x_vals: Vec<f64> = (0..n).map(|x| x as f64).collect();
    let y_vals = series;

    let x_mean = x_vals.iter().sum::<f64>() / n as f64;
    let y_mean = y_vals.iter().sum::<f64>() / n as f64;

    let numerator: f64 = x_vals
        .iter()
        .zip(y_vals)
        .map(|(x, y)| (x - x_mean) * (y - y_mean))
        .sum();

    let denominator: f64 = x_vals.iter().map(|x| (x - x_mean).powi(2)).sum();

    if denominator == 0.0 {
        return Err("Divisão por zero na regressão.");
    }

    let slope = numerator / denominator;
    let intercept = y_mean - slope * x_mean;

    Ok((slope, intercept))
}

/// Realiza previsão de n valores futuros com base no modelo linear.
pub fn predict(series: &[f64], slope: f64, intercept: f64, n: usize) -> Vec<f64> {
    let start_x = series.len();
    (0..n)
        .map(|i| {
            let x = (start_x + i) as f64;
            slope * x + intercept
        })
        .collect()
}

/// Calcula o Coeficiente de Determinação R².
pub fn r_squared(series: &[f64], slope: f64, intercept: f64) -> f64 {
    let y_mean = series.iter().sum::<f64>() / series.len() as f64;
    let ss_total: f64 = series.iter().map(|y| (y - y_mean).powi(2)).sum();

    let ss_res: f64 = series
        .iter()
        .enumerate()
        .map(|(i, y)| {
            let x = i as f64;
            let y_pred = slope * x + intercept;
            (y - y_pred).powi(2)
        })
        .sum();

    if ss_total == 0.0 {
        1.0 // todos os valores são iguais, o modelo explica tudo
    } else {
        1.0 - (ss_res / ss_total)
    }
}

/// Calcula o Erro Quadrático Médio (MSE).
pub fn mean_squared_error(series: &[f64], slope: f64, intercept: f64) -> f64 {
    let sum_squared_errors: f64 = series
        .iter()
        .enumerate()
        .map(|(i, y)| {
            let x = i as f64;
            let y_pred = slope * x + intercept;
            (y - y_pred).powi(2)
        })
        .sum();

    sum_squared_errors / series.len() as f64
}
