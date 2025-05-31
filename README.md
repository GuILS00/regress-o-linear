# TimeWise Analytics

> Sistema de análise de séries temporais com estatísticas descritivas, regressão linear, visualização em ASCII e geração de relatórios — implementado em Rust.

## 📌 Visão Geral

Este projeto acadêmico em Rust tem como objetivo ler séries temporais de arquivos CSV ou JSON, calcular estatísticas descritivas, aplicar regressão linear simples e gerar relatórios automatizados no terminal com previsão e métricas como R² e MSE. É ideal para análise rápida de dados de tendências temporais.

---

## ✅ Funcionalidades

- 📥 **Leitura de dados**
  - Suporte a arquivos `.csv` e `.json`
  - Estrutura de dados robusta para entrada temporal

- 📊 **Estatísticas descritivas**
  - Média, mediana, mínimo, máximo, desvio padrão

- 📈 **Regressão linear simples**
  - Ajuste da equação `y = a*x + b`
  - Previsão de valores futuros

- 🧮 **Métricas de avaliação**
  - Coeficiente de determinação (R²)
  - Erro quadrático médio (MSE)

- 🖥️ **Visualização ASCII**
  - Gráfico simples da série diretamente no terminal

- 📄 **Relatório no terminal**
  - Relatório com todos os dados calculados

- 🧪 **Testes**
  - Testes de integração no módulo `tests/`

---

## 📁 Estrutura do Projeto

TimeWise_Analytics/
├── data/
│ └── exemplo.json # Arquivo de exemplo
├── src/
│ ├── analysis.rs # Cálculo estatístico
│ ├── data.rs # Leitura de arquivos
│ ├── lib.rs # Gerenciador de módulos
│ ├── linear.rs # Regressão linear
│ ├── linear.regression.rs # Alternativa de módulo (não usada)
│ ├── main.rs # Interface interativa principal
│ ├── report.rs # Geração de relatório no terminal
│ ├── visual.rs # Gráfico ASCII
├── tests/
│ └── integration_tests.rs # Testes automatizados
├── relatorio.txt # Saída salva de exemplo do relatório
├── README.md # Este arquivo
├── Cargo.toml
└── Cargo.lock


---

## ▶️ Como Executar

1. **Clone o projeto**
```bash
git clone https://github.com/GuILS00/regress-o-linear.git
cd timewise_analytics

Compile
cargo build

Execute o sistema
cargo run

Execute os testes
cargo test

Exemplo de Entrada
Arquivo CSV (data.csv)
tempo,valor
2023-01,12.0
2023-02,15.5
2023-03,17.2

Arquivo JSON (exemplo.json)
[
  { "tempo": "2023-01", "valor": 12.0 },
  { "tempo": "2023-02", "valor": 15.5 },
  { "tempo": "2023-03", "valor": 17.2 }
]

📄 Exemplo de Saída
Estatísticas:
- Média: 14.90
- Mediana: 15.50
- Máximo: 17.20
- Mínimo: 12.00
- Desvio Padrão: 2.10

Regressão Linear: y = 2.60x + 9.40
MSE = 1.28
R² = 0.975

Previsão:
- Próximo valor estimado: 19.8

Gráfico ASCII:
   *
  **
 ***
----

📚 Dependências
serde – Serialização JSON

serde_json – Leitura de arquivos JSON

csv – Leitura de arquivos CSV

🎓 Projeto Acadêmico
Este projeto foi desenvolvido como parte de um trabalho acadêmico no curso de Análise e Desenvolvimento de Sistemas, abordando conceitos de análise de dados, regressão e programação em Rust.

👤 GuILS00
Seu Guilherme
GitHub 

