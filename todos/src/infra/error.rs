use crate::domain::error::{impl_from_trait, DomainError};

impl_from_trait!(diesel::r2d2::Error);
impl_from_trait!(diesel::r2d2::PoolError);
impl_from_trait!(diesel::result::Error);
