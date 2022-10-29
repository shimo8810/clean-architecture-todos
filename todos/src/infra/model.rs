use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use super::schema::tasks;
use crate::domain;
use crate::domain::error::DomainError;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct Task {
    pub id: String,
    pub body: String,
    pub state: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct NewTask {
    pub title: String,
}

impl TryFrom<Task> for domain::Task {
    type Error = DomainError;
    fn try_from(task: Task) -> Result<Self, Self::Error> {
        Ok(Self::new(
            domain::TaskId::from_str(&task.id)?,
            domain::TaskBody::from_str(&task.body)?,
            domain::TaskState::from(task.state),
        ))
    }
}
