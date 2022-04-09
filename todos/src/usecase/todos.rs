use crate::domain::{Task, TaskFilter, TaskId, TaskRepository};

pub struct UseCase<Repo: TaskRepository> {
    repository: Repo,
}

impl<Repo: TaskRepository> UseCase<Repo> {
    pub fn new(repository: Repo) -> Self {
        Self { repository }
    }

    pub fn get_task_list(&self, filter: TaskFilter) -> Result<Vec<Task>, String> {
        self.repository.list(filter)
    }

    pub fn add_task(&self, task: Task) -> Result<(), String> {
        self.repository.insert(task)
    }

    pub fn delete_task(&self, id: TaskId) -> Result<(), String> {
        self.repository.delete(id)
    }

    pub fn update_task(&self, task: Task) -> Result<(), String> {
        self.repository.update(task)
    }
}
