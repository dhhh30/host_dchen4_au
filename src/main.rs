use actix_web::middleware::Logger;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::sqlite::SqliteConnection;
use env_logger::Env;
use host::database_handler::*;
use std::fs;
//state passed to handlers
struct State {
    db_pool: Pool<ConnectionManager<SqliteConnection>>,
    auth_db_pool: Pool<ConnectionManager<SqliteConnection>>,
}

#[get("/")]
async fn index() -> impl Responder {
    let html = std::fs::read_to_string("assets/index.html").
    unwrap();
    HttpResponse::Ok().body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files as fs;

    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(State {
                db_pool: acquire_connection_pool(),
                auth_db_pool: acquire_authentication_pool(),
            }))
            .service(fs::Files::new("/assets", "./assets"))
            .service(index)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
