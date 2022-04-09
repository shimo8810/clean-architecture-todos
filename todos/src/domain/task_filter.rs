use super::TaskState;

#[derive(Debug)]
pub enum TaskFilter {
    All,
    StateEq(TaskState),
}
