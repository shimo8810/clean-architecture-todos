use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub enum TaskState {
    Active,
    Completed,
}

impl fmt::Display for TaskState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match *self {
            Self::Active => "Active",
            Self::Completed => "Completed",
        };

        write!(f, "{}", str)
    }
}

impl FromStr for TaskState {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &*s.to_lowercase() {
            "active" => Ok(Self::Active),
            "completed" => Ok(Self::Completed),
            _ => Err("incorrect string value".to_string()),
        }
    }
}
