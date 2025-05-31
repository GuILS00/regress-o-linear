/// Gera uma visualização simples (gráfico de linha) em ASCII da série temporal.
pub fn grafico_ascii(series: &[f64]) {
    if series.is_empty() {
        println!("Série vazia.");
        return;
    }

    let max = series.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let min = series.iter().cloned().fold(f64::INFINITY, f64::min);
    let altura = 10.0;

    for nivel in (0..=altura as usize).rev() {
        let limiar = min + (max - min) * nivel as f64 / altura;
        for &valor in series {
            if valor >= limiar {
                print!("█");
            } else {
                print!(" ");
            }
        }
        println!();
    }

    println!("{}", "-".repeat(series.len()));
}


