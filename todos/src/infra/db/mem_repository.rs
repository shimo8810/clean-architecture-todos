use crate::domain::TaskRepository;
use crate::domain::{Task, TaskFilter, TaskId};

#[derive(Debug, PartialEq, Clone)]
pub struct MemoTaskReposiory {
    tasks: Vec<Task>,
}
impl MemoTaskReposiory {
    pub fn new() -> Self {
        Self { tasks: vec![] }
    }
}

impl Default for MemoTaskReposiory {
    fn default() -> Self {
        Self::new()
    }
}

impl TaskRepository for MemoTaskReposiory {
    fn list(&self, filter: TaskFilter) -> Result<Vec<Task>, String> {
        Ok(match filter {
            TaskFilter::All => self.tasks.clone(),
            TaskFilter::StateEq(state) => self
                .tasks
                .iter()
                .filter(|&task| task.state == state)
                .cloned()
                .collect(),
        })
    }

    fn insert(&mut self, task: Task) -> Result<(), String> {
        self.tasks.push(task);
        Ok(())
    }

    fn delete(&mut self, id: TaskId) -> Result<(), String> {
        if let Some(idx) = self.tasks.iter().position(|task| task.id == id) {
            self.tasks.remove(idx);
        }

        Ok(())
    }

    fn update(&mut self, task: Task) -> Result<(), String> {
        if let Some(idx) = self.tasks.iter().position(|t| t.id == task.id) {
            self.tasks.get_mut(idx).unwrap().body = task.body;
            self.tasks.get_mut(idx).unwrap().state = task.state;
        }

        Ok(())
    }
}
