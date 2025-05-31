/// Módulo para análise estatística descritiva de séries temporais.

/// Calcula a média de um vetor de valores.
pub fn media(data: &[f64]) -> f64 {
    let sum: f64 = data.iter().sum();
    sum / data.len() as f64
}

/// Calcula a variância populacional.
pub fn variancia(data: &[f64]) -> f64 {
    let m = media(data);
    let sum_sq_diff: f64 = data.iter().map(|x| (x - m).powi(2)).sum();
    sum_sq_diff / data.len() as f64
}

/// Calcula o desvio padrão populacional.
pub fn desvio_padrao(data: &[f64]) -> f64 {
    variancia(data).sqrt()
}

/// Calcula o mínimo valor.
pub fn minimo(data: &[f64]) -> f64 {
    *data.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
}

/// Calcula o máximo valor.
pub fn maximo(data: &[f64]) -> f64 {
    *data.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
}

/// Calcula estatísticas básicas e retorna (média, desvio padrão, min, max).
pub fn estatisticas_basicas(data: &[f64]) -> (f64, f64, f64, f64) {
    (media(data), desvio_padrao(data), minimo(data), maximo(data))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_estatisticas_basicas() {
        let dados = [10.0, 20.0, 30.0, 40.0, 50.0];
        let (media, dp, min, max) = estatisticas_basicas(&dados);

        assert!((media - 30.0).abs() < 1e-6);
        assert!((dp - 14.1421356237).abs() < 1e-6);
        assert_eq!(min, 10.0);
        assert_eq!(max, 50.0);
    }
}
