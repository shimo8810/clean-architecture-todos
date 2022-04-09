use crate::domain::{Task, TaskFilter, TaskId, TaskRepository};
use anyhow::Result;
pub struct UseCase<Repo: TaskRepository> {
    repository: Repo,
}

impl<Repo: TaskRepository> UseCase<Repo> {
    pub fn new(repository: Repo) -> Self {
        Self { repository }
    }

    pub fn get_task_list(&self, filter: TaskFilter) -> Result<Vec<Task>> {
        self.repository.list(filter)
    }

    pub fn add_task(&self, task: Task) -> Result<()> {
        self.repository.insert(task)
    }

    pub fn delete_task(&self, id: TaskId) -> Result<()> {
        self.repository.delete(id)
    }

    pub fn update_task(&self, task: Task) -> Result<()> {
        self.repository.update(task)
    }
}
