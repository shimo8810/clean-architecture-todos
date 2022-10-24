#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TaskState {
    Active,
    Completed,
}

impl TaskState {
    pub fn to_bool(&self) -> bool {
        match self {
            TaskState::Active => false,
            TaskState::Completed => true,
        }
    }
}

impl From<bool> for TaskState {
    fn from(state: bool) -> Self {
        if state {
            TaskState::Completed
        } else {
            TaskState::Active
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_state() {
        assert_ne!(TaskState::Active, TaskState::Completed);
    }

    #[test]
    fn create_from_bool() {
        assert_eq!(TaskState::from(true), TaskState::Completed);
        assert_eq!(TaskState::from(false), TaskState::Active);
        assert!(!TaskState::Active.to_bool());
        assert!(TaskState::Completed.to_bool());
    }
}
