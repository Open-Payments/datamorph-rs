use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to parse specification: {0}")]
    SpecParseError(String),

    #[error("Transform error: {0}")]
    TransformError(String),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Other error: {0}")]
    Other(String),
}