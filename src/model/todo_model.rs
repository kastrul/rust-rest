use crate::model::DBResponse;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgQueryResult;
use sqlx::{FromRow, PgPool};

#[derive(Serialize, Deserialize)]
pub struct TodoRequest {
    pub description: String,
    pub done: bool,
}

#[derive(Serialize, FromRow)]
pub struct Todo {
    pub id: i32,
    pub description: String,
    pub done: bool,
}

impl Todo {
    pub async fn find_all(pool: &PgPool) -> DBResponse<Vec<Todo>> {
        let todos = sqlx::query_as!(Todo, "SELECT * FROM todo ORDER BY id")
            .fetch_all(pool)
            .await?;

        Ok(todos)
    }

    pub async fn find_by_id(id: i32, pool: &PgPool) -> DBResponse<Option<Todo>> {
        let todo = sqlx::query_as!(Todo, "SELECT * FROM todo WHERE id = $1", id)
            .fetch_optional(pool)
            .await?;

        Ok(todo)
    }

    pub async fn create(todo: TodoRequest, pool: &PgPool) -> DBResponse<Todo> {
        let todo = sqlx::query_as!(
            Todo,
            "INSERT INTO todo (description, done) VALUES ($1, $2) RETURNING *",
            todo.description,
            todo.done,
        )
        .fetch_one(pool)
        .await?;

        Ok(todo)
    }

    pub async fn update(id: i32, todo: TodoRequest, pool: &PgPool) -> DBResponse<Option<Todo>> {
        let todo = sqlx::query_as!(
            Todo,
            "UPDATE todo SET description = $1, done = $2 WHERE id = $3 RETURNING *",
            todo.description,
            todo.done,
            id,
        )
        .fetch_optional(pool)
        .await?;

        Ok(todo)
    }

    pub async fn delete(id: i32, pool: &PgPool) -> DBResponse<u64> {
        let n_deleted: PgQueryResult = sqlx::query!("DELETE FROM todo WHERE id = $1", id)
            .execute(pool)
            .await?;

        Ok(n_deleted.rows_affected())
    }
}
