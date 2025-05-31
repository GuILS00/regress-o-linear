use std::fs::File;
use std::io::{BufReader, BufRead};
use std::error::Error;

/// Estrutura para armazenar uma série temporal simples.
#[derive(Debug)]
pub struct SerieTemporal {
    pub tempo: Vec<String>,  // Datas ou marcadores de tempo como strings
    pub valores: Vec<f64>,   // Valores numéricos associados
}

/// Carrega uma série temporal a partir de um arquivo CSV com duas colunas: tempo, valor.
pub fn carregar_csv(caminho: &str) -> Result<SerieTemporal, Box<dyn Error>> {
    let file = File::open(caminho)?;
    let reader = BufReader::new(file);

    let mut tempo = Vec::new();
    let mut valores = Vec::new();

    for (i, linha_resultado) in reader.lines().enumerate() {
        let linha = linha_resultado?;
        if i == 0 && linha.contains("tempo") && linha.contains("valor") {
            continue; // pula o cabeçalho
        }

        let partes: Vec<&str> = linha.trim().split(',').collect();
        if partes.len() != 2 {
            continue; // ignora linhas mal formatadas
        }

        tempo.push(partes[0].to_string());
        valores.push(partes[1].parse::<f64>()?);
    }

    Ok(SerieTemporal { tempo, valores })
}

/// Carrega uma série temporal de um arquivo JSON no formato:
/// [ {"tempo": "2023-01", "valor": 123.4}, ... ]
pub fn carregar_json(caminho: &str) -> Result<SerieTemporal, Box<dyn Error>> {
    let file = File::open(caminho)?;
    let reader = BufReader::new(file);

    #[derive(serde::Deserialize)]
    struct Entrada {
        tempo: String,
        valor: f64,
    }

    let entradas: Vec<Entrada> = serde_json::from_reader(reader)?;
    let mut tempo = Vec::new();
    let mut valores = Vec::new();

    for entrada in entradas {
        tempo.push(entrada.tempo);
        valores.push(entrada.valor);
    }

    Ok(SerieTemporal { tempo, valores })
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::write;


    #[test]
    fn test_carregar_csv() {
        let conteudo = "tempo,valor\n2023-01,10.0\n2023-02,20.5\n2023-03,15.0";
        let caminho = "teste_dados.csv";
        write(caminho, conteudo).unwrap();

        let serie = carregar_csv(caminho).unwrap();
        assert_eq!(serie.tempo.len(), 3);
        assert_eq!(serie.valores.len(), 3);
        assert_eq!(serie.valores[0], 10.0);
        assert_eq!(serie.tempo[1], "2023-02");

        std::fs::remove_file(caminho).unwrap();
    }

    #[test]
    fn test_carregar_json() {
        let conteudo = r#"
        [
            {"tempo": "2023-01", "valor": 10.0},
            {"tempo": "2023-02", "valor": 20.5},
            {"tempo": "2023-03", "valor": 15.0}
        ]
        "#;
        let caminho = "teste_dados.json";
        write(caminho, conteudo).unwrap();

        let serie = carregar_json(caminho).unwrap();
        assert_eq!(serie.tempo.len(), 3);
        assert_eq!(serie.valores[2], 15.0);
        assert_eq!(serie.tempo[0], "2023-01");

        std::fs::remove_file(caminho).unwrap();
    }
}
