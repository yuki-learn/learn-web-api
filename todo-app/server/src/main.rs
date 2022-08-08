use actix_web::web;
use todo_app::{db_connection::create_pool, app_config::config_app};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    let pool = create_pool();
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(config_app)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}