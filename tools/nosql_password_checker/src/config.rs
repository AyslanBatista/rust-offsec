use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Target {
    pub host: String,
    pub protocol: String,
    pub path: String,
    pub port: String,
}

#[derive(Debug, Deserialize)]
pub struct Request {
    pub method: String,
    pub follow_redirects: bool,
}

#[derive(Debug, Deserialize)]
pub struct Headers {
    pub content_type: String,
    pub user_agent: String,
    #[serde(default)]
    pub referer: Option<String>,
    // Adicione outros headers conforme necessário
}

#[derive(Debug, Deserialize)]
pub struct Payload {
    #[serde(rename = "type")]
    pub type_: String,
    pub data: String,
}

#[derive(Debug, Deserialize)]
pub struct Output {
    pub show_body: bool,
    pub max_body_length: usize,
}

#[derive(Debug, Deserialize)]
pub struct Response {
    pub success: String,
 //   pub error: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub target: Target,
    pub request: Request,
    pub headers: Headers,
    pub payload: Payload,
    pub output: Output,
    pub response: Response,
}

pub fn load_config(config_path: Option<impl AsRef<Path>>) -> Result<Settings, ConfigError> {
    let mut builder = Config::builder()
        .add_source(File::with_name("config/default"));

    // Adiciona arquivo de configuração específico se fornecido
    if let Some(path) = config_path {
        builder = builder.add_source(File::from(path.as_ref()));
    }

    // Constrói a configuração
    let config = builder.build()?;
    
    // Converte para a estrutura Settings
    config.try_deserialize()
}
