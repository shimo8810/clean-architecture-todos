use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, PartialEq, Default)]
pub struct TaskId(Uuid);

impl TaskId {
    pub fn new() -> Self {
        TaskId(Uuid::new_v4())
    }
}

impl FromStr for TaskId {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Uuid::from_str(s) {
            Ok(id) => Ok(TaskId(id)),
            Err(e) => Err(e.to_string()),
        }
    }
}

impl fmt::Display for TaskId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn create_id() {
        let id1 = TaskId::new();
        let id2 = TaskId::default();
        let id3 = TaskId::default();
        assert_ne!(id1, id2);
        assert_eq!(id2, id3);
        assert_ne!(id3, id1);
    }

    #[test]
    fn create_from_str() {
        let str = "550e8400-e29b-41d4-a716-446655440000";
        let id = TaskId::from_str(str).unwrap();
        assert_eq!(id.to_string(), str);
    }
}
