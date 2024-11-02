use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Transform error: {0}")]
    Transform(String),

    #[error("JSONLogic error: {0}")]
    Logic(String),

    #[error("Missing field: {0}")]
    MissingField(String),
}