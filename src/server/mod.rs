pub mod error;
pub mod routers;

use actix_web::{web, App, HttpServer};

use crate::domain::TaskRepository;
use crate::usecase::task::UseCase;

#[actix_web::main]
pub async fn run<R: TaskRepository + Clone + Send + Sync + 'static>(
    usecase: UseCase<R>,
) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(usecase.clone()))
            .route("/tasks", web::get().to(routers::list_tasks::<R>))
            .route("/tasks", web::post().to(routers::post_task::<R>))
            .route("/tasks/{id}", web::put().to(routers::update_task::<R>))
            .route("tasks/{id}", web::delete().to(routers::delete_task::<R>))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
