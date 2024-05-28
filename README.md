# CLI

> Comando para executar:

```bash
cargo run -- input1.txt input2.txt -v
```

> Explicação:

Definição da Estrutura de Argumentos e Opções:

- Utilizamos a biblioteca Clap (Command Line Argument Parser) para definir a estrutura dos argumentos e opções do nosso programa de linha de comando.

Processamento de Expressões Regulares:

- Empregamos a biblioteca Regex (Regular Expressions) para identificar padrões específicos em linhas de texto, como o comando "printe" seguido de uma mensagem.

Leitura e Processamento de Arquivos de Entrada:

- Utilizamos o módulo std::fs::File e std::io para ler e processar os arquivos de entrada fornecidos pelo usuário.

Iteração sobre Linhas de Texto:

- Aproveitamos o sistema de iteradores em Rust para percorrer as linhas de texto em cada arquivo de entrada e realizar operações de filtragem e processamento.

Gestão de Dependências e Construção do Projeto:

- Utilizamos a ferramenta Cargo, que é o gerenciador de pacotes e construção de projetos padrão em Rust, para gerenciar as dependências do projeto, compilar o código-fonte e executar o programa.

### Ou em uma explicação linha a linha comentando o código:

```rust
use clap::{Arg, Command}; // Importa os structs Arg e Command do crate clap
use std::fs::File; // Importa o struct File do módulo fs do crate padrão std
use std::io::{self, BufRead}; // Importa os traits io e BufRead do módulo io do crate padrão std
use std::path::Path; // Importa o struct Path do módulo path do crate padrão std
use regex::Regex; // Importa o struct Regex do crate regex

fn main() {
    // Define a estrutura de argumentos e opções usando o crate clap
    let matches = Command::new("My CLI") // Define o nome do programa
        .about("Ferramenta CLI com suporte a múltiplos arquivos e filtros de linha") // Descrição do programa
        .arg(Arg::new("input") // Define o argumento 'input' para arquivos de entrada
            .help("Arquivos de entrada") // Descrição do argumento
            .required(true) // Argumento obrigatório
            .num_args(1..)) // Espera pelo menos um argumento
        .arg(Arg::new("verbose") // Define a opção 'verbose' para saída detalhada
            .short('v') // Atalho curto para a opção
            .long("verbose") // Atalho longo para a opção
            .help("Saída detalhada") // Descrição da opção
            .action(clap::ArgAction::SetTrue)) // Define a ação da opção
        .arg(Arg::new("filter") // Define a opção 'filter' para filtrar linhas
            .short('f') // Atalho curto para a opção
            .long("filter") // Atalho longo para a opção
            .help("Filtrar linhas que contêm o texto especificado") // Descrição da opção
            .value_parser(clap::value_parser!(String))) // Define o tipo de valor da opção
        .get_matches(); // Obtém os argumentos e opções passados pela linha de comando

    // Obtém os valores dos argumentos e opções
    let input_files: Vec<&String> = matches.get_many::<String>("input").unwrap().collect(); // Obtém os arquivos de entrada
    let verbose = matches.get_flag("verbose"); // Verifica se a opção 'verbose' foi ativada
    let filter = matches.get_one::<String>("filter"); // Obtém o valor da opção 'filter' se fornecido

    // Define um regex para capturar o comando "printe" e a mensagem
    let print_regex = Regex::new(r"printe\s+'([^']*)'").unwrap();

    // Itera sobre os arquivos de entrada
    for input in input_files {
        // Verifica se a saída detalhada está ativada e imprime o nome do arquivo sendo processado
        if verbose {
            println!("Processando arquivo: {}", input);
        }

        // Lê as linhas do arquivo e processa cada uma
        if let Ok(lines) = read_lines(input) {
            let mut line_count = 0; // Inicializa o contador de linhas
            for line in lines {
                if let Ok(content) = line {
                    // Verifica se a linha corresponde ao regex do comando "printe"
                    if let Some(caps) = print_regex.captures(&content) {
                        let message = &caps[1]; // Extrai a mensagem da linha
                        println!("{}", message); // Imprime a mensagem
                        continue; // Pula para a próxima iteração do loop
                    }

                    // Aplica o filtro se fornecido
                    if let Some(filter_text) = filter {
                        if content.contains(filter_text) {
                            println!("{}", content);
                        }
                    } else {
                        println!("{}", content); // Imprime a linha se não houver filtro
                    }
                    line_count += 1; // Incrementa o contador de linhas
                }
            }
            println!("Número de linhas em {}: {}", input, line_count); // Imprime o número de linhas processadas
        } else {
            eprintln!("Erro ao ler o arquivo: {}", input); // Imprime uma mensagem de erro se houver problemas ao ler o arquivo
        }
    }
}

// Função auxiliar para ler as linhas de um arquivo
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?; // Abre o arquivo
    Ok(io::BufReader::new(file).lines()) // Retorna um iterador sobre as linhas do arquivo
}
```
