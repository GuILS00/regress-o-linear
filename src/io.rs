use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Deserialize)]
pub struct Registro {
    pub tempo: f64,
    pub valor: f64,
}

pub fn carregar_dados_csv(caminho: &str) -> Result<Vec<Registro>, Box<dyn std::error::Error>> {
    let file = File::open(caminho)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut dados = Vec::new();

    for result in rdr.deserialize() {
        let registro: Registro = result?;
        dados.push(registro);
    }

    Ok(dados)
}

pub fn carregar_dados_json(caminho: &str) -> Result<Vec<Registro>, Box<dyn std::error::Error>> {
    let file = File::open(caminho)?;
    let reader = BufReader::new(file);
    let dados = serde_json::from_reader(reader)?;
    Ok(dados)
}
