use actix_web::{delete, get, HttpResponse, post, put, Responder, web};
use sqlx::PgPool;

use crate::model::todo_model::{Todo, TodoRequest};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all)
        .service(find)
        .service(create)
        .service(update)
        .service(delete);
}

#[get("/todo")]
async fn find_all(db_pool: web::Data<PgPool>) -> impl Responder {
    let result = Todo::find_all(&db_pool).await;
    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(err) => {
            log::error!("error fetching todos: {:?}", err);
            HttpResponse::InternalServerError()
                .body("Error trying to read all todos from database")
        }
    }
}

#[get("/todo/{id}")]
async fn find(id: web::Path<i32>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = Todo::find_by_id(*id, &db_pool).await;

    match result {
        Ok(Some(todo)) => HttpResponse::Ok().json(todo),
        Ok(None) => HttpResponse::NotFound().body("Todo not found"),
        Err(err) => {
            log::error!("error fetching todo: {:?}", err);
            HttpResponse::InternalServerError().body("Error trying to read todo from database")
        }
    }
}

#[post("/todo")]
async fn create(
    todo: web::Json<TodoRequest>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let result = Todo::create(todo.into_inner(), &db_pool).await;
    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(err) => {
            log::error!("error creating todo: {:?}", err);
            HttpResponse::InternalServerError().body("Error trying to create new todo")
        }
    }
}

#[put("/todo/{id}")]
async fn update(
    id: web::Path<i32>,
    todo: web::Json<TodoRequest>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let result = Todo::update(*id, todo.into_inner(), &db_pool).await;

    match result {
        Ok(Some(todo)) => HttpResponse::Ok().json(todo),
        Ok(None) => HttpResponse::NotFound().body("Todo not found"),
        Err(err) => {
            log::error!("error updating todo: {:?}", err);
            HttpResponse::InternalServerError().body("Error trying to update todo")
        }
    }
}

#[delete("/todo/{id}")]
async fn delete(id: web::Path<i32>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = Todo::delete(*id, &db_pool).await;

    match result {
        Ok(rows_deleted) => {
            if rows_deleted > 0 {
                let msg = format!("Successfully deleted {} record(s)", rows_deleted);
                HttpResponse::Ok().body(msg)
            } else {
                HttpResponse::NotFound().body("Todo not found")
            }
        }
        Err(err) => {
            log::error!("error deleting todo: {:?}", err);
            HttpResponse::InternalServerError().body("Todo not found")
        }
    }
}
