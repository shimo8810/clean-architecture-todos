use serde::{Deserialize, Serialize};
use std::str::FromStr;

use super::error::UseCaseError;
use crate::domain::{task::Task, task_body::TaskBody, task_id::TaskId, task_state::TaskState};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct TaskDto {
    id: String,
    body: String,
    state: bool,
}

impl From<Task> for TaskDto {
    fn from(task: Task) -> Self {
        Self {
            id: task.id.to_string(),
            body: task.body.to_string(),
            state: task.state.to_bool(),
        }
    }
}

impl TryFrom<TaskDto> for Task {
    type Error = UseCaseError;
    fn try_from(task: TaskDto) -> Result<Self, Self::Error> {
        Ok(Self::new(
            TaskId::from_str(&task.id)?,
            TaskBody::from_str(&task.body)?,
            TaskState::from(task.state),
        ))
    }
}
