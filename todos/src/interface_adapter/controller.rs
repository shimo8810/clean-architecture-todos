use crate::domain::{Task, TaskBody, TaskFilter, TaskId, TaskRepository, TaskState};
use crate::interface_adapter::request::{
    AddTaskRequest, DeleteTaskRequest, GetTaskRequest, UpdateTaskRequest,
};
use crate::interface_adapter::response::TaskResponse;
use crate::usecase::todos::UseCase;
use std::str::FromStr;
pub struct Controller<Repo: TaskRepository> {
    usecase: UseCase<Repo>,
}

impl<Repo: TaskRepository> Controller<Repo> {
    pub fn new(repository: Repo) -> Self {
        let usecase = UseCase::new(repository);
        Self { usecase }
    }

    pub fn get_task_list(&self, req: GetTaskRequest) -> Result<Vec<TaskResponse>, String> {
        let filter = req.filter;
        let filter = filter.map_or(TaskFilter::All, |s| match &*s {
            "Active" => TaskFilter::StateEq(TaskState::Active),
            "Completed" => TaskFilter::StateEq(TaskState::Active),
            _ => TaskFilter::All,
        });

        self.usecase
            .get_task_list(filter)
            .map(|tasks| tasks.into_iter().map(|task| task.into()).collect())
    }

    pub fn add_task(&mut self, req: AddTaskRequest) -> Result<(), String> {
        let body = TaskBody::from_str(&req.body)?;
        let task = Task::new(body);
        self.usecase.add_task(task)
    }

    pub fn delete_task(&mut self, req: DeleteTaskRequest) -> Result<(), String> {
        let id = TaskId::from_str(&req.id)?;
        self.usecase.delete_task(id)
    }

    pub fn update_task(&mut self, req: UpdateTaskRequest) -> Result<(), String> {
        let id = TaskId::from_str(&req.id)?;
        let body = TaskBody::from_str(&req.body)?;
        let state = TaskState::from_str(&req.state)?;
        let task = Task { id, state, body };
        self.usecase.update_task(task)
    }
}
