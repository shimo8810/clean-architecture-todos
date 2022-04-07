use actix_web::{web, HttpResponse, Result};

use crate::domain::TaskRepository;
use crate::infra::server::State;
use crate::interface_adapter::request::{
    AddTaskRequest, DeleteTaskRequest, GetTaskRequest, UpdateTaskRequest,
};

pub async fn get_tasks<R: TaskRepository>(
    data: web::Data<State<R>>,
    web::Query(query): web::Query<GetTaskRequest>,
) -> Result<HttpResponse> {
    let controller = data.controller.lock().unwrap();
    let ans = controller.get_task_list(query).unwrap();

    let response = HttpResponse::Ok().json(ans);
    Ok(response)
}

pub async fn post_task<R: TaskRepository>(
    data: web::Data<State<R>>,
    web::Query(query): web::Query<AddTaskRequest>,
) -> Result<HttpResponse> {
    let mut controller = data.controller.lock().unwrap();
    let ans = controller.add_task(query).unwrap();

    let response = HttpResponse::Ok().json(ans);
    Ok(response)
}

pub async fn delete_task<R: TaskRepository>(
    data: web::Data<State<R>>,
    web::Query(query): web::Query<DeleteTaskRequest>,
) -> Result<HttpResponse> {
    let mut controller = data.controller.lock().unwrap();
    let ans = controller.delete_task(query).unwrap();

    let response = HttpResponse::Ok().json(ans);
    Ok(response)
}

pub async fn put_task<R: TaskRepository>(
    data: web::Data<State<R>>,
    web::Query(query): web::Query<UpdateTaskRequest>,
) -> Result<HttpResponse> {
    let mut controller = data.controller.lock().unwrap();
    let ans = controller.update_task(query).unwrap();

    let response = HttpResponse::Ok().json(ans);
    Ok(response)
}
