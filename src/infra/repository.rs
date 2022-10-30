use super::model;
use super::schema;
use crate::domain;

use diesel::QueryDsl;
use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection, RunQueryDsl,
};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Clone)]
pub struct PgTaskRepository {
    pub pool: DbPool,
}

impl PgTaskRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl domain::TaskRepository for PgTaskRepository {
    fn list(&self) -> Result<Vec<domain::Task>, crate::domain::error::DomainError> {
        let mut conn = self.pool.get()?;
        let tasks: Vec<model::Task> = schema::tasks::table.load(&mut conn)?;

        tasks
            .into_iter()
            .map(domain::Task::try_from)
            .collect::<Result<Vec<_>, _>>()
    }

    fn insert(&self, task: &domain::Task) -> Result<(), crate::domain::error::DomainError> {
        let task = model::Task {
            id: task.id.to_string(),
            body: task.body.to_string(),
            state: task.state.to_bool(),
        };

        let mut conn = self.pool.get()?;
        diesel::insert_into(schema::tasks::table)
            .values(&task)
            .execute(&mut conn)?;

        Ok(())
    }

    fn delete(&self, id: &domain::TaskId) -> Result<(), crate::domain::error::DomainError> {
        let mut conn = self.pool.get()?;
        let id = id.to_string();
        diesel::delete(schema::tasks::table.find(id)).execute(&mut conn)?;

        Ok(())
    }

    fn update(&self, task: &domain::Task) -> Result<(), crate::domain::error::DomainError> {
        todo!()
    }
}
