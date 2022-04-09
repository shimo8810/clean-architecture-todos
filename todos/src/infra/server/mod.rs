pub mod handlers;
use crate::domain::TaskRepository;
use crate::interface_adapter::controller::Controller;
use actix_web::{web, App, HttpServer};
use anyhow::Result;

pub async fn run_server<R: TaskRepository + Send + Sync + 'static>(repo: R) -> Result<()> {
    let state = web::Data::new(Controller::new(repo));

    HttpServer::new(move || {
        App::new().app_data(state.clone()).service(
            web::resource("/tasks")
                .route(web::get().to(handlers::get_tasks::<R>))
                .route(web::post().to(handlers::post_task::<R>))
                .route(web::delete().to(handlers::delete_task::<R>))
                .route(web::put().to(handlers::put_task::<R>)),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
