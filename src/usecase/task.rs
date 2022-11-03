use std::str::FromStr;

use uuid::Uuid;

use super::error::UseCaseError;
use super::task_dto::{NewTaskDto, TaskDto, UpdateTaskDto};
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

    pub fn add_task(&self, new_task: NewTaskDto) -> Result<TaskDto, UseCaseError> {
        let task = Task::new(
            TaskId::new(Uuid::new_v4()),
            TaskBody::new(new_task.body)?,
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

    pub fn update_task(
        &self,
        id: &str,
        update_task: UpdateTaskDto,
    ) -> Result<TaskDto, UseCaseError> {
        let id = TaskId::from_str(id)?;
        let body = TaskBody::from_str(&update_task.body)?;
        let state = TaskState::from(update_task.state);
        let task = Task::new(id, body, state);

        self.repository.update(&task)?;

        Ok(task.into())
    }
}
