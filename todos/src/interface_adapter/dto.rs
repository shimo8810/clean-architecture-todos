use crate::domain::{Task, TaskBody, TaskId, TaskState};
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub struct TaskDto {
    pub id: String,
    pub state: String,
    pub body: String,
}

impl From<Task> for TaskDto {
    fn from(task: Task) -> Self {
        let id = task.id.to_string();
        let state = task.state.to_string();
        let body = task.body.to_string();

        Self { id, state, body }
    }
}

impl TryFrom<TaskDto> for Task {
    type Error = String;
    fn try_from(task: TaskDto) -> Result<Self, Self::Error> {
        let id = TaskId::from_str(&task.id)?;
        let body = TaskBody::from_str(&task.body)?;
        let state = TaskState::from_str(&task.state)?;
        Ok(Task { id, state, body })
    }
}
