use crate::usecase::error::UseCaseError;

impl From<UseCaseError> for actix_web::Error {
    fn from(e: UseCaseError) -> actix_web::Error {
        match &e {
            UseCaseError::Validation(_) => actix_web::error::ErrorBadRequest(e),
            UseCaseError::NotFound(_) => actix_web::error::ErrorNotFound(e),
            UseCaseError::Other(_) => actix_web::error::ErrorInternalServerError(e),
        }
    }
}
