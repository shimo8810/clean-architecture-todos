use super::{error::DomainError, task::Task, task_id::TaskId};

#[mockall::automock]
pub trait TaskRepository {
    fn list(&self) -> Result<Vec<Task>, DomainError>;

    fn insert(&self, task: &Task) -> Result<(), DomainError>;

    fn delete(&self, id: &TaskId) -> Result<(), DomainError>;
}
