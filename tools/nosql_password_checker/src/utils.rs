use crate::config::Output;
use colored::*;
use reqwest::blocking::Response;

pub fn print_response(response: Response, output_config: &Output) {
    // Imprime informações básicas da resposta
    println!(
        "{}: {} {}",
        "Status".green().bold(),
        response.status().as_u16().to_string().yellow(),
        response.status().canonical_reason().unwrap_or("").yellow()
    );

    println!(
        "{}: {}",
        "URL".green().bold(),
        response.url().to_string().blue()
    );

    // Imprime o corpo se configurado
    if output_config.show_body {
        // Obtém o texto do corpo da resposta
        match response.text() {
            Ok(body) => {
                if !body.is_empty() {
                    println!("\n{}", "Corpo da resposta:".green().bold());

                    // Limita o tamanho do corpo se necessário
                    if body.len() > output_config.max_body_length
                        && output_config.max_body_length > 0
                    {
                        println!("{}\n...", &body[..output_config.max_body_length]);
                        println!(
                            "({} caracteres truncados)",
                            body.len() - output_config.max_body_length
                        );
                    } else {
                        println!("{}", body);
                    }
                } else {
                    println!("\n{}", "Corpo da resposta vazio".yellow());
                }
            }
            Err(e) => println!("{}: {}", "Erro ao ler corpo da resposta".red(), e),
        }
    }
}
