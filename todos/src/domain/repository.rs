use super::{Task, TaskFilter, TaskId};
use anyhow::Result;
pub trait TaskRepository {
    fn list(&self, filter: TaskFilter) -> Result<Vec<Task>>;

    fn insert(&self, task: Task) -> Result<()>;

    fn delete(&self, id: TaskId) -> Result<()>;

    fn update(&self, task: Task) -> Result<()>;
}
