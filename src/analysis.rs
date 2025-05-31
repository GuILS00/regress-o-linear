pub fn calcular_media(valores: &[f64]) -> f64 {
    let soma: f64 = valores.iter().sum();
    soma / valores.len() as f64
}

pub fn calcular_mediana(valores: &mut [f64]) -> f64 {
    valores.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let meio = valores.len() / 2;
    if valores.len() % 2 == 0 {
        (valores[meio - 1] + valores[meio]) / 2.0
    } else {
        valores[meio]
    }
}

pub fn calcular_desvio_padrao(valores: &[f64]) -> f64 {
    let media = calcular_media(valores);
    let variancia = valores.iter().map(|v| (v - media).powi(2)).sum::<f64>() / valores.len() as f64;
    variancia.sqrt()
}

pub fn calcular_min_max(valores: &[f64]) -> (f64, f64) {
    let min = valores
        .iter()
        .cloned()
        .fold(f64::INFINITY, |a, b| a.min(b));
    let max = valores
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, |a, b| a.max(b));
    (min, max)
}

pub fn estatisticas_basicas(valores: &[f64]) {
    let soma: f64 = valores.iter().sum();
    let media = soma / valores.len() as f64;

    let variancia = valores.iter()
        .map(|v| (v - media).powi(2))
        .sum::<f64>() / valores.len() as f64;

    let desvio_padrao = variancia.sqrt();

    println!("\n📊 Estatísticas:");
    println!("Média: {:.2}", media);
    println!("Desvio padrão: {:.2}", desvio_padrao);
}
