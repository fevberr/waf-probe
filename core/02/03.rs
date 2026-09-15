use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error0 {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("http: {0}")]
    Http(#[from] reqwest::Error),
    #[error("yaml: {0}")]
    Yaml(#[from] serde_yaml::Error),
    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
    #[error("url: {0}")]
    Url(#[from] url::ParseError),
    #[error("regex: {0}")]
    Regex(#[from] regex::Error),
    #[error("auth not confirmed")]
    Auth,
    #[error("config: {0}")]
    Config(String),
}

pub type Result0<T> = std::result::Result<T, Error0>;