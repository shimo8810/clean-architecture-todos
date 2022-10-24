use super::{task_body::TaskBody, task_id::TaskId, task_state::TaskState};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Task {
    pub id: TaskId,
    pub body: TaskBody,
    pub state: TaskState,
}

impl Task {
    pub fn new(id: TaskId, body: TaskBody, state: TaskState) -> Self {
        Self { id, body, state }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use uuid::Uuid;

    #[test]
    fn create_task() {
        let uid = Uuid::new_v4();
        let id = TaskId::new(uid);
        let state = TaskState::Active;
        let body = TaskBody::new("body").unwrap();

        let task = Task { id, state, body };

        assert_eq!(task.id.to_string(), uid.to_string());
        assert_eq!(task.state, TaskState::Active);
        assert_eq!(task.body, TaskBody::new("body").unwrap());
    }
}
