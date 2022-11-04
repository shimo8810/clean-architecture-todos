use crate::domain::error::DomainError;

#[derive(Debug, thiserror::Error)]
pub enum UseCaseError {
    #[error("{0}")]
    Validation(String),
    #[error("ID {0} not found")]
    NotFound(String),
    #[error(transparent)]
    Other(anyhow::Error),
}

impl From<DomainError> for UseCaseError {
    fn from(e: DomainError) -> UseCaseError {
        match e {
            DomainError::Validation(s) => UseCaseError::Validation(s),
            DomainError::NotFound(i) => UseCaseError::NotFound(i),
            DomainError::Other(err) => UseCaseError::Other(err),
        }
    }
}
