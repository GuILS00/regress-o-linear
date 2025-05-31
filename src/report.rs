use std::fs::File;
use std::io::Write;
use crate::Registro;

pub fn gerar_relatorio(
    caminho: &str,
    media: f64,
    desvio: f64,
    a: f64,
    b: f64,
    r2: f64,
    mse: f64,
    dados: &[Registro],
    previsoes: &[f64],
) -> std::io::Result<()> {
    let mut arquivo = File::create("relatorio.txt")?;

    writeln!(arquivo, "# Relatório de Análise Temporal")?;
    writeln!(arquivo, "\n📁 Arquivo analisado: {}", caminho)?;
    writeln!(arquivo, "\n📊 Estatísticas:")?;
    writeln!(arquivo, "- Média: {:.2}", media)?;
    writeln!(arquivo, "- Desvio padrão: {:.2}", desvio)?;

    writeln!(arquivo, "\n📈 Regressão Linear:")?;
    writeln!(arquivo, "- Fórmula: y = {:.2}x + {:.2}", a, b)?;
    writeln!(arquivo, "- R²: {:.4}", r2)?;
    writeln!(arquivo, "- MSE: {:.4}", mse)?;

    writeln!(arquivo, "\n🔮 Previsões:")?;
    for (i, y_pred) in previsoes.iter().enumerate() {
        writeln!(arquivo, "{} => {:.2}", dados[i].tempo, y_pred)?;
    }

    writeln!(arquivo, "\n✅ Fim do relatório.")?;
    Ok(())
}
