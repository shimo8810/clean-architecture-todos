#[derive(thiserror::Error, Debug)]
pub enum DomainError {
    #[error("{0}")]
    Validation(String),
    #[error("ID {0} not found")]
    NotFound(String),
    #[error(transparent)]
    Other(anyhow::Error),
}

#[macro_export]
macro_rules! impl_from_trait {
    ($etype: ty) => {
        impl From<$etype> for DomainError {
            fn from(e: $etype) -> Self {
                DomainError::Other(anyhow::anyhow!(e))
            }
        }
    };
}

pub use impl_from_trait;
