/// Implementa regressão linear simples, cálculo de métricas e previsão.

/// Realiza a regressão linear simples sobre os dados de entrada.
/// Retorna o coeficiente angular (a) e o coeficiente linear (b).
pub fn linear_regression(x: &[f64], y: &[f64]) -> Result<(f64, f64), &'static str> {
    if x.len() != y.len() || x.is_empty() {
        return Err("Entradas inválidas. Vetores devem ter o mesmo tamanho e não podem estar vazios.");
    }

    let n = x.len() as f64;
    let soma_x: f64 = x.iter().sum();
    let soma_y: f64 = y.iter().sum();
    let soma_xy: f64 = x.iter().zip(y.iter()).map(|(xi, yi)| xi * yi).sum();
    let soma_x2: f64 = x.iter().map(|xi| xi * xi).sum();

    let denominador = n * soma_x2 - soma_x * soma_x;
    if denominador == 0.0 {
        return Err("Denominador zero na regressão. Não é possível ajustar a reta.");
    }

    let a = (n * soma_xy - soma_x * soma_y) / denominador;
    let b = (soma_y - a * soma_x) / n;

    Ok((a, b))
}

/// Calcula o Coeficiente de Determinação R².
pub fn calcular_r2(x: &[f64], y: &[f64], a: f64, b: f64) -> f64 {
    let media_y: f64 = y.iter().sum::<f64>() / y.len() as f64;
    let ss_total: f64 = y.iter().map(|yi| (yi - media_y).powi(2)).sum();
    let ss_residual: f64 = x.iter().zip(y.iter()).map(|(xi, yi)| (yi - (a * xi + b)).powi(2)).sum();

    1.0 - (ss_residual / ss_total)
}

/// Calcula o Erro Quadrático Médio (MSE).
pub fn calcular_mse(x: &[f64], y: &[f64], a: f64, b: f64) -> f64 {
    let erro_quadratico: f64 = x.iter().zip(y.iter())
        .map(|(xi, yi)| (yi - (a * xi + b)).powi(2))
        .sum();
    erro_quadratico / x.len() as f64
}

/// Realiza previsões com base nos coeficientes a e b.
pub fn prever_valores(x_novos: &[f64], a: f64, b: f64) -> Vec<f64> {
    x_novos.iter().map(|xi| a * xi + b).collect()
}
