use crate::domain::{Task, TaskBody, TaskFilter, TaskId, TaskRepository, TaskState};
use crate::interface_adapter::request::{
    AddTaskRequest, DeleteTaskRequest, GetTaskRequest, UpdateTaskRequest,
};
use crate::interface_adapter::response::TaskResponse;
use crate::usecase::todos::UseCase;
use anyhow::Result;

use std::str::FromStr;

pub struct Controller<Repo: TaskRepository> {
    usecase: UseCase<Repo>,
}

impl<Repo: TaskRepository> Controller<Repo> {
    pub fn new(repository: Repo) -> Self {
        let usecase = UseCase::new(repository);
        Self { usecase }
    }

    pub fn get_task_list(&self, req: GetTaskRequest) -> Result<Vec<TaskResponse>> {
        let filter = req
            .filter
            .map_or(TaskFilter::All, |s| match &*s.to_lowercase() {
                "active" => TaskFilter::StateEq(TaskState::Active),
                "completed" => TaskFilter::StateEq(TaskState::Completed),
                _ => TaskFilter::All,
            });

        self.usecase
            .get_task_list(filter)
            .map(|tasks| tasks.into_iter().map(|task| task.into()).collect())
    }

    pub fn add_task(&self, req: AddTaskRequest) -> Result<()> {
        let body = TaskBody::from_str(&req.body)?;
        let task = Task::new(body);
        self.usecase.add_task(task)
    }

    pub fn delete_task(&self, req: DeleteTaskRequest) -> Result<()> {
        let id = TaskId::from_str(&req.id)?;
        self.usecase.delete_task(id)
    }

    pub fn update_task(&self, req: UpdateTaskRequest) -> Result<()> {
        let id = TaskId::from_str(&req.id)?;
        let body = TaskBody::from_str(&req.body)?;
        let state = TaskState::from_str(&req.state)?;
        let task = Task { id, state, body };
        self.usecase.update_task(task)
    }
}
