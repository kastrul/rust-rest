use std::env;

use actix_web::{App, HttpResponse, HttpServer, middleware, Responder, web};
use actix_web_httpauth::middleware::HttpAuthentication;
use dotenv::dotenv;
use sqlx::migrate::{MigrateError, Migrator};
use sqlx::PgPool;

use crate::security::authentication::ok_validator;
use crate::security::tls::get_tls_config;
use crate::service::todo_service;

mod service;
mod model;
mod security;

static MIGRATOR: Migrator = sqlx::migrate!("db/migrations");

async fn index() -> impl Responder {
    HttpResponse::Ok().body(
        r#"
        Welcome to Actix-web with SQLx Todos example.
        Available routes:
        GET /todos -> list of all todos
        POST /todo -> create new todo, example: { "description": "learn actix and sqlx", "done": false }
        GET /todo/{id} -> show one todo with requested id
        PUT /todo/{id} -> update todo with requested id, example: { "description": "learn actix and sqlx", "done": true }
        DELETE /todo/{id} -> delete todo with requested id
        "#
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file")
        .parse::<u16>()
        .expect("PORT should be a u16");

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    log::info!("connecting to postgres DB at: {}", &db_url);
    let db_pool = PgPool::connect(&db_url).await.expect("DB connection failed");
    let migrate_res: Result<_, MigrateError> = MIGRATOR.run(&db_pool).await;
    match migrate_res {
        Err(e) => log::error!("migrate failed: {:?}", e),
        Ok(_) => log::info!("Migrations successful"),
    }

    let tls_config = get_tls_config().expect("TLS configuration failed.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .wrap(middleware::Logger::default())
            .wrap(HttpAuthentication::bearer(ok_validator))
            .route("/", web::get().to(index))
            .configure(todo_service::init)
    }).bind_rustls((host, port), tls_config)?
        .run()
        .await
}
