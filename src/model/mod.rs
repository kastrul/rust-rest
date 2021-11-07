pub mod todo_model;

type DBResponse<T> = Result<T, sqlx::Error>;
