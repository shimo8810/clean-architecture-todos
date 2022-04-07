use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct GetTaskRequest {
    pub filter: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct AddTaskRequest {
    pub body: String,
}

#[derive(Deserialize, Serialize)]
pub struct DeleteTaskRequest {
    pub id: String,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateTaskRequest {
    pub id: String,
    pub body: String,
    pub state: String,
}
