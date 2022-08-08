use actix_web::web;
use crate::handlers::todos;

pub fn config_app(cfg: &mut web::ServiceConfig) {
    // /api/todos/{id}/
    cfg.service(
        web::scope("/api/v1")
            .service(
                web::scope("/todos")
                    .service(
                        web::resource("")
                            .route(web::get().to(todos::find_all))
                            .route(web::post().to(todos::post_todo)),
                    )
                    .service(
                        web::scope("/{id}")
                            .service(
                                web::resource("")
                                    .route(web::get().to(todos::find_by))
                                    .route(web::delete().to(todos::delete_todo))
                                    .route(web::put().to(todos::put_todo))
                            )
                    ),
            )
    );
}