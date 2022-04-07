use std::fmt;
use std::str::FromStr;

const MAX_LENGTH: usize = 140;

#[derive(Debug, PartialEq, Clone)]
pub struct TaskBody(String);

impl TaskBody {
    pub fn new<S: ToString>(body: S) -> Result<Self, String> {
        let body = body.to_string();
        if body.len() > MAX_LENGTH {
            Err("too long todo's body".to_string())
        } else {
            Ok(Self(body))
        }
    }
}

impl FromStr for TaskBody {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TaskBody::new(s)
    }
}

impl fmt::Display for TaskBody {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_body() {
        let body_text = "some task";
        let body = TaskBody::new(body_text);
        assert!(body.is_ok());
        assert_eq!(body.unwrap().to_string(), body_text);
    }

    #[test]
    fn long_body_error() {
        let body_text = "0123456789".to_string().repeat(17);
        assert!(body_text.len() > MAX_LENGTH);
        let body = TaskBody::new(body_text);
        assert!(body.is_err());
    }
}
