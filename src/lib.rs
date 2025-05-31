/// Módulo principal da TimeWise Analytics
/// Contém importações e exposições públicas dos componentes da biblioteca.

pub mod data;         // Importação de CSV/JSON
pub mod analysis;     // Análise estatística descritiva
pub mod linear;       // Regressão linear e previsão
pub mod report;       // Geração de relatórios
pub mod visual;
pub mod io; // ou o nome do arquivo onde estão as funções de leitura CSV/JSON
pub mod utils;


// Reexporta para uso direto em main.rs
pub use io::{carregar_dados_csv, carregar_dados_json, Registro};
pub use linear::{linear_regression, prever_valores, calcular_r2, calcular_mse};
pub use analysis::estatisticas_basicas;
pub use report::gerar_relatorio;
pub use analysis::{calcular_media, calcular_mediana, calcular_desvio_padrao, calcular_min_max};
// Reexporta funções úteis diretamente do módulo linear
pub use visual::grafico_ascii;

#[test]
fn teste_grafico_ascii() {
    let dados = [1.0, 2.0, 3.0, 2.0, 5.0, 8.0, 6.0, 3.0, 1.0];
    crate::visual::grafico_ascii(&dados);
}
