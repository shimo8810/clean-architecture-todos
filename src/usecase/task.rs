use std::str::FromStr;

use uuid::Uuid;

use super::error::UseCaseError;
use super::task_dto::TaskDto;
use crate::domain::{
    repository::TaskRepository, task::Task, task_body::TaskBody, task_id::TaskId,
    task_state::TaskState,
};

#[derive(Debug, Clone)]
pub struct UseCase<R: TaskRepository> {
    repository: R,
}

impl<R: TaskRepository> UseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn list_tasks(&self) -> Result<Vec<TaskDto>, UseCaseError> {
        Ok(self
            .repository
            .list()?
            .into_iter()
            .map(TaskDto::from)
            .collect())
    }

    pub fn add_task(&self, body: &str) -> Result<TaskDto, UseCaseError> {
        let task = Task::new(
            TaskId::new(Uuid::new_v4()),
            TaskBody::new(body)?,
            TaskState::Active,
        );

        self.repository.insert(&task)?;

        Ok(task.into())
    }

    pub fn delete_task(&self, id: &str) -> Result<(), UseCaseError> {
        let id = TaskId::from_str(id)?;
        self.repository.delete(&id)?;
        Ok(())
    }
}
