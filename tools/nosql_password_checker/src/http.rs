use crate::config::Settings;
use reqwest::blocking::{Client, Response};
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue, REFERER, USER_AGENT};
use reqwest::redirect::Policy;
use std::error::Error;

pub fn execute_request(settings: &Settings) -> Result<Response, Box<dyn Error>> {
    // Configura os headers
    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_str(&settings.headers.content_type)?,
    );
    headers.insert(
        USER_AGENT,
        HeaderValue::from_str(&settings.headers.user_agent)?,
    );

    if let Some(ref referer) = settings.headers.referer {
        headers.insert(REFERER, HeaderValue::from_str(referer)?);
    }

    // Monta a URL completa
    let mut url = format!("{}://{}", settings.target.protocol, settings.target.host);
    if !settings.target.port.is_empty() {
        url = format!("{}:{}", url, settings.target.port);
    }
    url = format!("{}{}", url, settings.target.path);

    // Configura a política de redirecionamento
    let redirect_policy = if settings.request.follow_redirects {
        Policy::default()
    } else {
        Policy::none()
    };

    // Cria o cliente HTTP
    let client = Client::builder().redirect(redirect_policy).build()?;

    // Executa a requisição com base no método
    let request_builder = match settings.request.method.to_uppercase().as_str() {
        "GET" => client.get(&url),
        "POST" => client.post(&url),
        "PUT" => client.put(&url),
        "DELETE" => client.delete(&url),
        "HEAD" => client.head(&url),
        "PATCH" => client.patch(&url),
        _ => return Err("Método HTTP não suportado".into()),
    };

    // Configura os headers e payload
    let request_with_headers = request_builder.headers(headers);

    // Adiciona o payload com base no tipo
    let response = match settings.payload.type_.as_str() {
        "form" => request_with_headers
            .body(settings.payload.data.clone())
            .send()?,
        "json" => {
            // Parseamos o JSON da string
            let json_value: serde_json::Value = serde_json::from_str(&settings.payload.data)?;
            request_with_headers.json(&json_value).send()?
        }
        "raw" | _ => request_with_headers
            .body(settings.payload.data.clone())
            .send()?,
    };

    Ok(response)
}
