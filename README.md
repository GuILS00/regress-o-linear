🕒 TimeWise Analytics
Um sistema em Rust para análise e previsão de séries temporais usando regressão linear, com suporte a CSV e JSON, visualização ASCII e geração de relatório automático.

🚀 Funcionalidades
📂 Leitura de séries temporais de arquivos .csv ou .json.

📊 Cálculo de média e desvio padrão dos valores.

📈 Regressão linear simples para prever valores futuros.

🔍 Exibição das previsões geradas.

🖼️ Gráficos ASCII da série original e da previsão.

📝 Geração de relatório completo (relatorio.txt) com:

Caminho do arquivo analisado

Estatísticas básicas

Fórmula da regressão

Métricas (R² e MSE)

Lista de previsões por timestamp

📦 Requisitos
Rust

Cargo (já vem com o Rust)

📁 Estrutura do Projeto

.
├── src/
│   ├── main.rs          # Entrada principal interativa
│   ├── io.rs            # (opcional) Funções de leitura
│   ├── linear.rs        # Regressão linear + métricas
│   ├── visual.rs        # Gráficos ASCII
│   └── report.rs        # Geração do relatório
├── data/
│   ├── exemplo.csv      # Exemplo de arquivo CSV
│   └── exemplo.json     # Exemplo de arquivo JSON
├── relatorio.txt        # Relatório gerado após execução
├── Cargo.toml
└── README.md

📂 Como usar
git clone https://github.com/seu-usuario/timewise-analytics.git
cd timewise-analytics
cargo run

Digite o caminho do arquivo quando solicitado, por exemplo:
data/exemplo.csv

📷 Exemplo de uso

📊 Estatísticas:
Média: 25.76
Desvio padrão: 4.12

📈 Regressão Linear:
Fórmula: y = 0.93x + 20.12
Coeficiente de determinação (R²): 0.8421
Erro quadrático médio (MSE): 5.4123

🔍 Previsões:
2024-01 => 20.12
2024-02 => 21.05
...

🖼️ Gráfico ASCII da série real:
█ █ █ ▓ ▓ ░ ░ ▒ ▒ ░ ░

📃 Licença
Projeto acadêmico. Livre para uso educacional.

✅ Status
✅ Concluído com todos os requisitos funcionais implementados.

⌨️ Autoria
Desenvolvido por Guilherme da Lapa Serra para o projeto TimeWise Analytics (2025).