use super::{TaskBody, TaskId, TaskState};

#[derive(Debug, PartialEq)]
pub struct Task {
    id: TaskId,
    state: TaskState,
    body: TaskBody,
}

impl Task {
    pub fn new(id: TaskId, state: TaskState, body: TaskBody) -> Self {
        Self { id, state, body }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn create_task() {
        let id = TaskId::default();
        let state = TaskState::Active;
        let body = TaskBody::new("body").unwrap();

        let task = Task::new(id, state, body);
        assert_eq!(task.id, TaskId::default());
        assert_eq!(task.state, TaskState::Active);
        assert_eq!(task.body, TaskBody::new("body").unwrap());
    }
}
