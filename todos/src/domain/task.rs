use super::{TaskBody, TaskId, TaskState};

#[derive(Debug, PartialEq, Clone)]
pub struct Task {
    pub id: TaskId,
    pub state: TaskState,
    pub body: TaskBody,
}

impl Task {
    pub fn new(body: TaskBody) -> Self {
        let id = TaskId::new();
        let state = TaskState::Active;
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

        let task = Task { id, state, body };
        assert_eq!(task.id, TaskId::default());
        assert_eq!(task.state, TaskState::Active);
        assert_eq!(task.body, TaskBody::new("body").unwrap());
    }
}
