use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::sqlite::SqliteConnection;
use env_logger::Env;
use host::database_handler::*;
use host::templating::*;
use log::*;
//state passed to handlers
struct State {
    db_pool: Pool<ConnectionManager<SqliteConnection>>,
    auth_db_pool: Pool<ConnectionManager<SqliteConnection>>,
}

#[get("/")]
async fn index() -> impl Responder {
    let html = std::fs::read_to_string("assets/index.html").
    unwrap();
    debug!("value of html: {}", html);
    HttpResponse::Ok().body(html)
}
//new alternative index
#[get("/new/index")]
async fn index_new() -> impl Responder {
    let index_str = index_template();
    debug!("value of index_str: {}", index_str);
    HttpResponse::Ok().body(index_str) 
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init_from_env(Env::default().default_filter_or("debug"));
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(State {
                db_pool: acquire_connection_pool(),
                auth_db_pool: acquire_authentication_pool(),
            }))
            .service(index)
            .service(index_new)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
