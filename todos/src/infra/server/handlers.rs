use actix_web::{web, HttpResponse, Result};

use crate::domain::TaskRepository;
// use crate::infra::server::State;
use crate::interface_adapter::controller::Controller;
use crate::interface_adapter::request::{
    AddTaskRequest, DeleteTaskRequest, GetTaskRequest, UpdateTaskRequest,
};

pub async fn get_tasks<R: TaskRepository>(
    controller: web::Data<Controller<R>>,
    web::Query(query): web::Query<GetTaskRequest>,
) -> Result<HttpResponse> {
    // let controller = data.controller;
    let resp = if let Ok(vec) = controller.get_task_list(query) {
        HttpResponse::Ok().json(vec)
    } else {
        HttpResponse::InternalServerError().finish()
    };

    Ok(resp)
}

pub async fn post_task<R: TaskRepository>(
    controller: web::Data<Controller<R>>,
    web::Query(query): web::Query<AddTaskRequest>,
) -> Result<HttpResponse> {
    // let response = HttpResponse::Ok().json(ans);
    let resp = if let Ok(_) = controller.add_task(query) {
        HttpResponse::Ok().body("create task")
    } else {
        HttpResponse::InternalServerError().finish()
    };
    Ok(resp)
}

pub async fn delete_task<R: TaskRepository>(
    controller: web::Data<Controller<R>>,
    web::Query(query): web::Query<DeleteTaskRequest>,
) -> Result<HttpResponse> {
    let resp = if let Ok(_) = controller.delete_task(query) {
        HttpResponse::Ok().body("delete task")
    } else {
        HttpResponse::InternalServerError().finish()
    };
    Ok(resp)
}

pub async fn put_task<R: TaskRepository>(
    controller: web::Data<Controller<R>>,
    web::Query(query): web::Query<UpdateTaskRequest>,
) -> Result<HttpResponse> {
    let resp = if let Ok(_) = controller.update_task(query) {
        HttpResponse::Ok().body("create task")
    } else {
        HttpResponse::InternalServerError().finish()
    };
    Ok(resp)
}
