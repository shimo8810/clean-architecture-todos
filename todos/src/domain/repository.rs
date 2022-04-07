use super::{Task, TaskFilter, TaskId};

pub trait TaskRepository {
    fn list(&self, filter: &TaskFilter) -> Result<Vec<Task>, String>;

    fn insert(&self, task: &Task) -> Result<(), String>;

    fn delete(&self, id: &TaskId) -> Result<(), String>;

    fn update(&self, task: &Task) -> Result<(), String>;
}
