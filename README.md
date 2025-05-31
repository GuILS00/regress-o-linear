# TimeWise Analytics

> Sistema de análise de séries temporais com estatísticas descritivas, regressão linear, visualização em ASCII e geração de relatórios — 100% em Rust.

---

## 📌 Visão Geral

Este projeto acadêmico em Rust tem como objetivo ler séries temporais de arquivos CSV ou JSON, calcular estatísticas descritivas, aplicar regressão linear simples e gerar relatórios automatizados no terminal com previsão e métricas como R² e MSE. Também oferece uma visualização gráfica ASCII e entrada interativa.

Ideal para análise rápida de tendências temporais com foco em desempenho, simplicidade e clareza.

---

## ✅ Funcionalidades

- 📥 **Leitura de dados**
  - Suporte a arquivos `.csv` e `.json`
  - Entrada interativa pelo terminal

- 📊 **Estatísticas descritivas**
  - Média, mediana, mínimo, máximo, desvio padrão

- 📈 **Regressão linear simples**
  - Ajuste da equação `y = a*x + b`
  - Previsão de valores futuros

- 🧮 **Métricas de avaliação**
  - Coeficiente de determinação (R²)
  - Erro quadrático médio (MSE)

- 🖥️ **Visualização ASCII**
  - Gráfico simples da série no terminal

- 📄 **Relatório gerado**
  - Exibido no terminal e salvo no arquivo `relatorio.txt`

- 🧪 **Testes**
  - Testes automatizados no módulo `tests/`

---

## 📁 Estrutura do Projeto


timewise_analytics/
├── data/
│   ├── exemplo.csv              # Exemplo de entrada em CSV
│   └── exemplo.json             # Exemplo de entrada em JSON
├── src/
│   ├── analysis.rs              # Cálculo estatístico
│   ├── data.rs                  # Leitura e parsing de dados
│   ├── lib.rs                   # Módulo central
│   ├── linear.rs                # Regressão linear oficial
│   ├── linear.regression.rs     # Alternativa (não usada, pode ser ignorada)
│   ├── main.rs                  # Interface interativa do terminal
│   ├── report.rs                # Geração de relatório e persistência
│   └── visual.rs                # Geração do gráfico ASCII
├── target/                      # Gerado automaticamente (ignorado pelo Git)
├── tests/
│   └── integration_tests.rs     # Testes automatizados
├── .gitignore                   # Arquivos ignorados pelo Git
├── Cargo.lock                   # Lockfile do projeto
├── Cargo.toml                   # Configurações e dependências
├── README.md                    # Documentação do projeto
└── relatorio.txt                # Relatório gerado (output persistido)


---

## ▶️ Como Executar

1. **Clone o projeto**

```bash
git clone https://github.com/GuILS00/regress-o-linear.git
cd regress-o-linear/timewise_analytics

Compile o projeto
cargo build

Execute o sistema
cargo run

Execute os testes
cargo test

📥 Exemplos de Entrada
Arquivo CSV (data.csv)
csv

tempo,valor
2023-01,12.0
2023-02,15.5
2023-03,17.2

Arquivo JSON (exemplo.json)
json
[
  { "tempo": "2023-01", "valor": 12.0 },
  { "tempo": "2023-02", "valor": 15.5 },
  { "tempo": "2023-03", "valor": 17.2 }
]

📤 Exemplo de Saída
Estatísticas:

Média: 14.90

Mediana: 15.50

Máximo: 17.20

Mínimo: 12.00

Desvio Padrão: 2.10

Regressão Linear: y = 2.60x + 9.40
MSE: 1.28
R²: 0.975

Previsão:

Próximo valor estimado: 19.8

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
Este projeto foi desenvolvido como parte de um trabalho acadêmico no curso de Análise e Desenvolvimento de Sistemas, abordando conceitos de análise de dados, regressão e programação com Rust puro (sem frameworks externos).

👤 Autor
Guilherme "GuILS00"
🔗 github.com/GuILS00