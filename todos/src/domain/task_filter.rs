use super::TaskState;

pub enum TaskFilter {
    All,
    StateEq(TaskState),
}
