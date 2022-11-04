use crate::usecase::task_dto::{NewTaskDto, UpdateTaskDto};

use crate::domain::TaskRepository;
use crate::usecase::task::UseCase;
use actix_web::{web, Error, HttpResponse};

pub async fn list_tasks<R: TaskRepository>(
    usecase: web::Data<UseCase<R>>,
) -> Result<HttpResponse, Error> {
    let list = usecase.list_tasks()?;

    Ok(HttpResponse::Ok().json(list))
}

pub async fn post_task<R: TaskRepository>(
    new_task: web::Json<NewTaskDto>,
    usecase: web::Data<UseCase<R>>,
) -> Result<HttpResponse, Error> {
    let new_task = new_task.into_inner();
    let task = usecase.add_task(&new_task)?;
    Ok(HttpResponse::Ok().json(task))
}

pub async fn delete_task<R: TaskRepository>(
    id: web::Path<String>,
    usecase: web::Data<UseCase<R>>,
) -> Result<HttpResponse, Error> {
    let id = id.into_inner();
    usecase.delete_task(&id)?;
    Ok(HttpResponse::Ok().body(format!("delete id: {}", id)))
}

pub async fn update_task<R: TaskRepository>(
    id: web::Path<String>,
    task: web::Json<UpdateTaskDto>,
    usecase: web::Data<UseCase<R>>,
) -> Result<HttpResponse, Error> {
    let task = task.into_inner();
    let task = usecase.update_task(&id, &task)?;

    Ok(HttpResponse::Ok().json(task))
}
