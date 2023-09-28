extern crate clap;
use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Contador de Palavras e Caracteres")
        .version("1.0")
        .author("Seu Nome")
        .about("Conta o número de palavras e caracteres em um arquivo de texto (.txt)")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("ARQUIVO")
                .help("Especifique o arquivo de entrada (.txt)")
                .required(true),
        )
        .get_matches();

    let input_file = matches.value_of("input").unwrap();
    let path = Path::new(input_file);

    if let Some(ext) = path.extension() {
        if ext == "txt" {
            processar_arquivo_texto(&path)?;
        } else {
            eprintln!("Formato de arquivo não suportado. Este programa aceita apenas arquivos .txt.");
        }
    } else {
        eprintln!("Extensão de arquivo não encontrada.");
    }

    Ok(())
}

fn processar_arquivo_texto(path: &Path) -> Result<(), Box<dyn Error>> {
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut word_count = 0;
    let mut char_count = 0;

    for line in reader.lines() {
        let line = line?;
        let words = line.split_whitespace();
        word_count += words.count();
        char_count += line.chars().filter(|c| !c.is_whitespace()).count();
    }

    println!("Número de palavras: {}", word_count);
    println!("Número de caracteres (sem espaços): {}", char_count);

    Ok(())
}
