use std::io;
use timewise_analytics::{linear_regression, calcular_r2, calcular_mse, prever_valores};

fn main() {
    println!("=== TimeWise Analytics: Análise de Série Temporal ===\n");

    // Entrada da série temporal
    println!("Digite os valores da série temporal, separados por espaço:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erro ao ler entrada");

    let y: Vec<f64> = input
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse::<f64>().ok())
        .collect();

    if y.len() < 2 {
        println!("A série precisa conter ao menos dois valores.");
        return;
    }

    let x: Vec<f64> = (0..y.len()).map(|i| i as f64).collect();

    match linear_regression(&x, &y) {
        Ok((a, b)) => {
            println!("\n--- Resultado da Regressão Linear ---");
            println!("Coeficiente Angular (slope): {:.4}", a);
            println!("Coeficiente Linear (intercept): {:.4}", b);

            let r2 = calcular_r2(&x, &y, a, b);
            let mse = calcular_mse(&x, &y, a, b);

            println!("\n--- Avaliação do Modelo ---");
            println!("Coeficiente de Determinação (R²): {:.4}", r2);
            println!("Erro Quadrático Médio (MSE): {:.4}", mse);

            println!("\nDigite quantos pontos futuros deseja prever:");
            let mut n_input = String::new();
            io::stdin().read_line(&mut n_input).expect("Erro ao ler entrada");

            let n: usize = match n_input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Entrada inválida.");
                    return;
                }
            };

            let novos_x: Vec<f64> = (x.len()..x.len() + n).map(|i| i as f64).collect();
            let previsoes = prever_valores(&novos_x, a, b);

            println!("\n--- Previsões Futuras ---");
            for (i, val) in previsoes.iter().enumerate() {
                println!("Ponto {}: {:.4}", x.len() + i, val);
            }
        }
        Err(e) => println!("Erro: {}", e),
    }
}
