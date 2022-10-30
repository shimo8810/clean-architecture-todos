use actix_web::{web, App, Error, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};

use crate::domain::TaskRepository;
use crate::usecase::task::UseCase;
use crate::usecase::task_dto::TaskDto;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
struct NewTask {
    body: String,
}

async fn list_tasks<R: TaskRepository>(
    usecase: web::Data<UseCase<R>>,
) -> Result<HttpResponse, Error> {
    let list = usecase
        .list_tasks()
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(list))
}

async fn post_task<R: TaskRepository>(
    data: web::Json<NewTask>,
    usecase: web::Data<UseCase<R>>,
) -> Result<HttpResponse, Error> {
    let task = usecase
        .add_task(&data.body)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(task))
}

async fn delete_task<R: TaskRepository>(
    id: web::Path<String>,
    usecase: web::Data<UseCase<R>>,
) -> Result<HttpResponse, Error> {
    let id = id.into_inner();
    usecase
        .delete_task(&id)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().body(format!("delete id: {}", id)))
}

async fn update_task<R: TaskRepository>(
    task: web::Json<TaskDto>,
    usecase: web::Data<UseCase<R>>,
) -> Result<HttpResponse, Error> {
    let task = task.into_inner();

    usecase
        .update_task(task)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().body("update"))
}

#[actix_web::main]
pub async fn run<R: TaskRepository + Clone + Send + Sync + 'static>(
    usecase: UseCase<R>,
) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(usecase.clone()))
            .route("/tasks", web::get().to(list_tasks::<R>))
            .route("/tasks", web::post().to(post_task::<R>))
            .route("/tasks", web::put().to(update_task::<R>))
            .route("tasks/{id}", web::delete().to(delete_task::<R>))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
