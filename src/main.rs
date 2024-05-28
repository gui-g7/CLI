use clap::{Arg, Command};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn main() {
    let matches = Command::new("My CLI")
        .about("Ferramenta CLI com suporte a múltiplos arquivos e filtros de linha")
        .arg(Arg::new("input")
            .help("Arquivos de entrada")
            .required(true)
            .num_args(1..))
        .arg(Arg::new("verbose")
            .short('v')
            .long("verbose")
            .help("Saída detalhada")
            .action(clap::ArgAction::SetTrue)) 
        .arg(Arg::new("filter")
            .short('f')
            .long("filter")
            .help("Filtrar linhas que contêm o texto especificado")
            .value_parser(clap::value_parser!(String))) 
        .get_matches();

    let input_files: Vec<&String> = matches.get_many::<String>("input").unwrap().collect();
    let verbose = matches.get_flag("verbose");
    let filter = matches.get_one::<String>("filter");

    let print_regex = Regex::new(r"printe\s+'([^']*)'").unwrap();

    for input in input_files {
        if verbose {
            println!("Processando arquivo: {}", input);
        }

        if let Ok(lines) = read_lines(input) {
            let mut line_count = 0;
            for line in lines {
                if let Ok(content) = line {
                    if let Some(caps) = print_regex.captures(&content) {
                        let message = &caps[1];
                        println!("{}", message);
                        continue;
                    }
                    
                    if let Some(filter_text) = filter {
                        if content.contains(filter_text) {
                            println!("{}", content);
                        }
                    } else {
                        println!("{}", content);
                    }
                    line_count += 1;
                }
            }
            println!("Número de linhas em {}: {}", input, line_count);
        } else {
            eprintln!("Erro ao ler o arquivo: {}", input);
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
