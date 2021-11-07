use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use sqlx::postgres::PgQueryResult;

type Res<T> = Result<T, sqlx::Error>;

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
    pub async fn find_all(pool: &PgPool) -> Res<Vec<Todo>> {
        let todos = sqlx::query_as!(Todo, "SELECT * FROM todo ORDER BY id")
            .fetch_all(pool)
            .await?;

        Ok(todos)
    }

    pub async fn find_by_id(id: i32, pool: &PgPool) -> Res<Option<Todo>> {
        let todo = sqlx::query_as!(Todo, "SELECT * FROM todo WHERE id = $1", id)
            .fetch_optional(pool)
            .await?;

        Ok(todo)
    }

    pub async fn create(todo: TodoRequest, pool: &PgPool) -> Res<Todo> {
        let mut tx = pool.begin().await?;
        let id_rec = sqlx::query!("INSERT INTO todo (description, done) VALUES ($1, $2) RETURNING id",
            todo.description,
            todo.done,
        ).fetch_one(&mut tx).await?;

        let rec = sqlx::query!("SELECT * FROM todo WHERE id = $1", id_rec.id)
            .fetch_one(&mut tx).await?;

        tx.commit().await?;

        Ok(Todo {
            id: rec.id,
            description: rec.description,
            done: rec.done,
        })
    }

    pub async fn update(id: i32, todo: TodoRequest, pool: &PgPool) -> Res<Option<Todo>> {
        let mut tx = pool.begin().await.unwrap();
        let n: PgQueryResult = sqlx::query!("UPDATE todo SET description = $1, done = $2 WHERE id = $3",
            todo.description,
            todo.done,
            id,
        ).execute(&mut tx).await?;

        if n.rows_affected() == 0 {
            return Ok(None);
        }

        let todo = sqlx::query_as!(Todo, "SELECT * FROM todo WHERE id = $1", id)
            .fetch_one(&mut tx)
            .await?;

        tx.commit().await.unwrap();
        Ok(Some(todo))
    }

    pub async fn delete(id: i32, pool: &PgPool) -> Res<u64> {
        let mut tx = pool.begin().await?;
        let n_deleted: PgQueryResult = sqlx::query!("DELETE FROM todo WHERE id = $1", id)
            .execute(&mut tx)
            .await?;

        tx.commit().await?;
        Ok(n_deleted.rows_affected())
    }
}
