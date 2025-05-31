/// Gera um gráfico ASCII vertical de uma série temporal.
/// `data`: vetor de valores da série.
/// `height`: número de linhas verticais para a escala do gráfico.
pub fn plot_ascii_series(data: &[f64], height: usize) {
    if data.is_empty() {
        println!("Nenhum dado para plotar.");
        return;
    }

    let min = data.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let range = max - min;

    let height = height.max(1);

    for row in (0..height).rev() {
        let threshold = min + range * (row as f64) / (height as f64);
        for &val in data {
            if val >= threshold {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }

    // Linha de base com índices
    println!("{}", (0..data.len()).map(|i| if i % 5 == 0 { '+' } else { '-' }).collect::<String>());
}
