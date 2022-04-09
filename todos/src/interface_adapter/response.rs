use crate::domain::{Task, TaskBody, TaskId, TaskState};
use anyhow::Error;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct TaskResponse {
    pub id: String,
    pub state: String,
    pub body: String,
}

impl From<Task> for TaskResponse {
    fn from(task: Task) -> Self {
        let id = task.id.to_string();
        let state = task.state.to_string();
        let body = task.body.to_string();

        Self { id, state, body }
    }
}

impl TryFrom<TaskResponse> for Task {
    type Error = Error;
    fn try_from(task: TaskResponse) -> Result<Self, Self::Error> {
        let id = TaskId::from_str(&task.id)?;
        let body = TaskBody::from_str(&task.body)?;
        let state = TaskState::from_str(&task.state)?;
        Ok(Task { id, state, body })
    }
}
