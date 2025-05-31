/// Módulo principal da TimeWise Analytics
/// Contém importações e exposições públicas dos componentes da biblioteca.

pub mod data;         // Importação de CSV/JSON
pub mod analysis;     // Análise estatística descritiva
pub mod linear;       // Regressão linear e previsão
pub mod report;       // Geração de relatórios
pub mod visual;

// Reexporta funções úteis diretamente do módulo linear
pub use linear::{linear_regression, calcular_r2, calcular_mse, prever_valores};
pub use visual::grafico_ascii;

#[test]
fn teste_grafico_ascii() {
    let dados = [1.0, 2.0, 3.0, 2.0, 5.0, 8.0, 6.0, 3.0, 1.0];
    crate::visual::grafico_ascii(&dados);
}
