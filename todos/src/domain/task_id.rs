use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

use crate::domain::error::DomainError;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TaskId(Uuid);

impl TaskId {
    pub fn new(id: Uuid) -> Self {
        Self(id)
    }
}

impl FromStr for TaskId {
    type Err = DomainError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Uuid::from_str(s) {
            Ok(id) => Ok(TaskId(id)),
            Err(e) => Err(DomainError::Other(anyhow::anyhow!(e))),
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
        let uid1 = Uuid::new_v4();
        let uid2 = Uuid::new_v4();
        assert_eq!(TaskId::new(uid1), TaskId::new(uid1));
        assert_ne!(TaskId::new(uid1), TaskId::new(uid2));
    }

    #[test]
    fn create_from_str() {
        let str = "550e8400-e29b-41d4-a716-446655440000";
        let id = TaskId::from_str(str).unwrap();
        assert_eq!(id.to_string(), str);
    }
}
