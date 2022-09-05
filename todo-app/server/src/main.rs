use std::env;

use actix_web::web;
use todo_app::{db_connection::create_pool, app_config::config_app};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    let pool = create_pool();
    let port: i32 = env::var("PORT")
        .unwrap_or_else(|_| "8088".to_string())
        .parse()
        .expect("PORT must be a number");

    let bind = format!("0.0.0.0:{}", port);
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(config_app)
    })
    .bind(bind)?
    .run()
    .await
}