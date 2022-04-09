use crate::domain::TaskRepository;
use crate::domain::{Task, TaskFilter, TaskId};
use std::sync::Mutex;
#[derive(Debug)]
pub struct MemoTaskReposiory {
    tasks: Mutex<Vec<Task>>,
}
impl MemoTaskReposiory {
    pub fn new() -> Self {
        Self {
            tasks: Mutex::new(vec![]),
        }
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
            TaskFilter::All => self.tasks.lock().unwrap().clone(),
            TaskFilter::StateEq(state) => self
                .tasks
                .lock()
                .unwrap()
                .iter()
                .filter(|&task| task.state == state)
                .cloned()
                .collect(),
        })
    }

    fn insert(&self, task: Task) -> Result<(), String> {
        self.tasks.lock().unwrap().push(task);
        Ok(())
    }

    fn delete(&self, id: TaskId) -> Result<(), String> {
        if let Some(idx) = self
            .tasks
            .lock()
            .unwrap()
            .iter()
            .position(|task| task.id == id)
        {
            self.tasks.lock().unwrap().remove(idx);
        }

        Ok(())
    }

    fn update(&self, task: Task) -> Result<(), String> {
        if let Some(idx) = self
            .tasks
            .lock()
            .unwrap()
            .iter()
            .position(|t| t.id == task.id)
        {
            self.tasks.lock().unwrap().get_mut(idx).unwrap().body = task.body;
            self.tasks.lock().unwrap().get_mut(idx).unwrap().state = task.state;
        }

        Ok(())
    }
}
