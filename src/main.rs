use timewise_analytics::gerar_relatorio;
use timewise_analytics::visual::plot_ascii_series;
use std::fs::File;
use std::io::{self, BufReader, Write};
use csv::ReaderBuilder;
use serde_json::from_reader;
use timewise_analytics::Registro;

mod linear;
mod visual;
use linear::*;
use visual::*;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    print!("Digite o caminho do arquivo (ex: data/exemplo.csv ou .json): ");
    io::stdout().flush()?; // Garante que o prompt apareça

    let mut caminho = String::new();
    io::stdin().read_line(&mut caminho)?;
    let caminho = caminho.trim();

    let arquivo = File::open(caminho)?;
    let reader = BufReader::new(arquivo);

    let mut dados: Vec<Registro> = Vec::new();

    if caminho.ends_with(".csv") {
        let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(reader);
        for result in rdr.deserialize() {
            let registro: Registro = result?;
            dados.push(registro);
        }
        println!("\n✅ Dados lidos do CSV:");
    } else if caminho.ends_with(".json") {
        dados = from_reader(reader)?;
        println!("\n✅ Dados lidos do JSON:");
    } else {
        println!("❌ Formato de arquivo não suportado.");
        return Ok(());
    }

    for r in &dados {
        println!("{:?}", r);
    }

    // Extrair valores
    let y: Vec<f64> = dados.iter().map(|r| r.valor).collect();
    let x: Vec<f64> = (0..y.len()).map(|i| i as f64).collect();

    // Estatísticas básicas
    let media = y.iter().sum::<f64>() / y.len() as f64;
    let desvio = (y.iter().map(|v| (v - media).powi(2)).sum::<f64>() / y.len() as f64).sqrt();

    println!("\n📊 Estatísticas:");
    println!("Média: {:.2}", media);
    println!("Desvio padrão: {:.2}", desvio);

    // Adiciona o gráfico vertical com '*'
println!("\n📊 Gráfico vertical (plot_ascii_series):");
plot_ascii_series(&y, 10);
    
    // Regressão
    if let Ok((a, b)) = linear_regression(&x, &y) {
        let previsoes = prever_valores(&x, a, b);
        let r2 = calcular_r2(&x, &y, a, b);
        let mse = calcular_mse(&x, &y, a, b);

        println!("\n📈 Regressão Linear:");
        println!("Fórmula: y = {:.2}x + {:.2}", a, b);
        println!("Coeficiente de determinação (R²): {:.4}", r2);
        println!("Erro quadrático médio (MSE): {:.4}", mse);

        println!("\n🔍 Previsões:");
        for (i, y_pred) in previsoes.iter().enumerate() {
            println!("{} => {:.2}", dados[i].tempo, y_pred);
        }

        println!("\n🖼️ Gráfico ASCII da série real:");
grafico_ascii(&y);

println!("\n🖼️ Gráfico ASCII da previsão:");
grafico_ascii(&previsoes);

// Geração de relatório
if let Err(e) = gerar_relatorio(
    caminho,
    media,
    desvio,
    a,
    b,
    r2,
    mse,
    &dados[..],
    &previsoes[..],
) {
    eprintln!("Erro ao gerar relatório: {}", e);
} else {
    println!("\n📝 Relatório salvo como 'relatorio.txt'");
}


        println!("\n🖼️ Qual gráfico deseja visualizar?");
println!("1. Gráfico de barras (█) - grafico_ascii");
println!("2. Gráfico de pontos (*) - plot_ascii_series");
print!("Digite o número da opção desejada: ");
io::stdout().flush()?; // Mostra o prompt

let mut escolha = String::new();
io::stdin().read_line(&mut escolha)?;
let escolha = escolha.trim();

match escolha {
    "1" => {
        println!("\n📊 Gráfico real (barras):");
        grafico_ascii(&y);
        println!("\n📊 Gráfico previsão (barras):");
        grafico_ascii(&previsoes);
    },
    "2" => {
        println!("\n📊 Gráfico real (pontos):");
        plot_ascii_series(&y, 10);
        println!("\n📊 Gráfico previsão (pontos):");
        plot_ascii_series(&previsoes, 10);
    },
    _ => println!("❌ Opção inválida. Nenhum gráfico exibido."),
}

    } else {
        println!("❌ Erro ao calcular a regressão.");
    }


    Ok(())
}

