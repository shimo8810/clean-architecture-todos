use crate::usecase::task_dto::{NewTaskDto, UpdateTaskDto};

use crate::domain::TaskRepository;
use crate::usecase::task::UseCase;
use actix_web::{web, Error, HttpResponse};

pub async fn list_tasks<R: TaskRepository>(
    usecase: web::Data<UseCase<R>>,
) -> Result<HttpResponse, Error> {
    let list = usecase
        .list_tasks()
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(list))
}

pub async fn post_task<R: TaskRepository>(
    data: web::Json<NewTaskDto>,
    usecase: web::Data<UseCase<R>>,
) -> Result<HttpResponse, Error> {
    let task = usecase
        .add_task(data.into_inner())
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(task))
}

pub async fn delete_task<R: TaskRepository>(
    id: web::Path<String>,
    usecase: web::Data<UseCase<R>>,
) -> Result<HttpResponse, Error> {
    let id = id.into_inner();
    usecase
        .delete_task(&id)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().body(format!("delete id: {}", id)))
}

pub async fn update_task<R: TaskRepository>(
    id: web::Path<String>,
    task: web::Json<UpdateTaskDto>,
    usecase: web::Data<UseCase<R>>,
) -> Result<HttpResponse, Error> {
    let task = task.into_inner();

    let task = usecase
        .update_task(&id, task)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(task))
}
