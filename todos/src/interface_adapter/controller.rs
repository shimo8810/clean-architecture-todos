use crate::domain::{Task, TaskBody, TaskFilter, TaskId, TaskRepository, TaskState};
use crate::interface_adapter::dto::TaskDto;
use crate::usecase::todos::UseCase;
use std::str::FromStr;

pub struct Controller<'r, Repo: TaskRepository> {
    usecase: UseCase<'r, Repo>,
}

impl<'r, Repo: TaskRepository> Controller<'r, Repo> {
    pub fn new(repository: &'r Repo) -> Self {
        let usecase = UseCase::new(repository);
        Self { usecase }
    }

    pub fn get_task_list(&self, filter: Option<&str>) -> Result<Vec<TaskDto>, String> {
        let filter = filter.map_or(TaskFilter::All, |s| match s {
            "Active" => TaskFilter::StateEq(TaskState::Active),
            "Completed" => TaskFilter::StateEq(TaskState::Active),
            _ => TaskFilter::All,
        });

        self.usecase
            .get_task_list(&filter)
            .map(|tasks| tasks.into_iter().map(|task| task.into()).collect())
    }

    pub fn add_task(&self, body: &str) -> Result<(), String> {
        let body = TaskBody::from_str(body)?;
        let task = Task::new(body);
        self.usecase.add_task(&task)
    }

    pub fn delete_task(&self, id: &str) -> Result<(), String> {
        let id = TaskId::from_str(id)?;
        self.usecase.delete_task(&id)
    }

    pub fn update_task(&self, task: TaskDto) -> Result<(), String> {
        let task = Task::try_from(task)?;
        self.usecase.update_task(&task)
    }
}
