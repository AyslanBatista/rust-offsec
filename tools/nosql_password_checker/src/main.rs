mod config;
mod http;
mod utils;
mod checker;

use clap::{Parser, Subcommand};
use colored::*;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Caminho para o arquivo de configuração
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Executa uma requisição HTTP
    Request {
        /// Host alvo (substitui o valor do arquivo de configuração)
        #[arg(long, short = 'H')]
        host: Option<String>,
        
        /// Caminho da requisição (substitui o valor do arquivo de configuração)
        #[arg(short, long)]
        path: Option<String>,
        
        /// Payload a ser enviado (substitui o valor do arquivo de configuração)
        #[arg(short, long)]
        payload: Option<String>,
    },
    /// Descobre a senha através de NoSQL Injection
    Bruteforce {
        /// Host alvo (substitui o valor do arquivo de configuração)
        #[arg(long, short = 'H')]
        host: Option<String>,
        
        /// Caminho da requisição (substitui o valor do arquivo de configuração)
        #[arg(short, long)]
        path: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();
    
    // Carrega a configuração
    let config_path = cli.config.as_deref();
    let mut settings = config::load_config(config_path).unwrap_or_else(|e| {
        eprintln!("{}: {}", "Erro ao carregar configuração".red().bold(), e);
        std::process::exit(1);
    });
    
    // Processa o comando
    match &cli.command {
        Some(Commands::Request { host, path, payload }) => {
            // Sobrescreve configurações com argumentos da linha de comando
            if let Some(h) = host {
                settings.target.host = h.clone();
            }
            
            if let Some(p) = path {
                settings.target.path = p.clone();
            }
            
            if let Some(data) = payload {
                settings.payload.data = data.clone();
            }
            
            // Executa a requisição
            match http::execute_request(&settings) {
                Ok(response) => {
                    // Verificamos redirecionamentos antes de passar para print_response
                    if let Some(location) = response.headers().get("location") {
                        println!(
                            "\n{}: {}",
                            "Redirecionamento para".magenta().bold(),
                            location.to_str().unwrap_or("").blue().underline()
                        );
                    }
                    
                    utils::print_response(response, &settings.output);
                }
                Err(e) => {
                    eprintln!("{}: {}", "Erro na requisição".red().bold(), e);
                }
            }
        },
        Some(Commands::Bruteforce { host, path }) => {
            // Sobrescreve configurações com argumentos da linha de comando
            if let Some(h) = host {
                settings.target.host = h.clone();
            }
            
            if let Some(p) = path {
                settings.target.path = p.clone();
            }
            
            // Primeiro determina o tamanho da senha
            match checker::checking_password_length(&mut settings) {
                Ok(length) => {
                    // Depois encontra cada caractere
                    match checker::finding_each_character(&mut settings, length) {
                        Ok(password) => {
                            println!("\n{}: {}", "🎉 Senha encontrada".green().bold(), password);
                        },
                        Err(e) => {
                            eprintln!("{}: {}", "Erro ao encontrar caracteres da senha".red().bold(), e);
                        }
                    }
                },
                Err(e) => {
                    eprintln!("{}: {}", "Erro ao determinar tamanho da senha".red().bold(), e);
                }
            }
        },
        None => {
            println!("{}", "Use --help para ver os comandos disponíveis".yellow());
        }
    }
}
