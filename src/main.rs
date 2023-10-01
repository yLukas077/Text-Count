use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Word and Character Counter")
        .version("1.0")
        .author("Your Name")
        .about("Counts the number of words and characters in a text file (.txt)")
        .arg(
            Arg::with_name("input")
                .short('i')
                .long("input")
                .value_name("FILE")
                .help("Specify the input file (.txt)")
                .required(true),
        )
        .get_matches();

    let input_file = matches.value_of("input").unwrap();
    let path = Path::new(input_file);

    if let Some(ext) = path.extension() {
        if ext == "txt" {
            process_text_file(&path)?;
        } else {
            eprintln!("Unsupported file format. This program only accepts .txt files.");
        }
    } else {
        eprintln!("File extension not found.");
    }

    Ok(())
}

fn process_text_file(path: &Path) -> Result<(), Box<dyn Error>> {
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

    println!("Number of words: {}", word_count);
    println!("Number of characters (excluding spaces): {}", char_count);

    Ok(())
}
