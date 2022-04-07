use super::{Task, TaskFilter, TaskId};

pub trait TaskRepository {
    fn list(&self, filter: TaskFilter) -> Result<Vec<Task>, String>;

    fn insert(&mut self, task: Task) -> Result<(), String>;

    fn delete(&mut self, id: TaskId) -> Result<(), String>;

    fn update(&mut self, task: Task) -> Result<(), String>;
}
