#[derive(thiserror::Error, Debug)]
pub enum DomainError {
    #[error("{0}")]
    Validation(String),
    #[error("task not found")]
    NotFound,
    #[error(transparent)]
    Other(anyhow::Error),
}
