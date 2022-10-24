use crate::domain::error::DomainError;

#[derive(Debug, thiserror::Error)]
pub enum UseCaseError {
    #[error("{0}")]
    Validation(String),
    #[error("task not found")]
    NotFound,
    #[error(transparent)]
    Other(anyhow::Error),
}

impl From<DomainError> for UseCaseError {
    fn from(e: DomainError) -> UseCaseError {
        match e {
            DomainError::Validation(s) => UseCaseError::Validation(s),
            DomainError::NotFound => UseCaseError::NotFound,
            DomainError::Other(err) => UseCaseError::Other(err),
        }
    }
}
