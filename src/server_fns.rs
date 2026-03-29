use crate::model::Todo;
use leptos::*;

#[cfg(feature = "ssr")]
fn db_err(e: sqlx::Error) -> ServerFnError {
    ServerFnError::ServerError(e.to_string())
}

#[server(GetTodos, "/api")]
pub async fn get_todos() -> Result<Vec<Todo>, ServerFnError> {
    use sqlx::SqlitePool;
    let pool = expect_context::<SqlitePool>();
    let todos = sqlx::query_as::<_, Todo>(
        "SELECT id, title, completed FROM todos ORDER BY created_at ASC",
    )
    .fetch_all(&pool)
    .await
    .map_err(db_err)?;
    Ok(todos)
}

#[server(AddTodo, "/api")]
pub async fn add_todo(title: String) -> Result<Todo, ServerFnError> {
    use sqlx::SqlitePool;
    let title = title.trim().to_string();
    if title.is_empty() {
        return Err(ServerFnError::ServerError(
            "Title cannot be empty".to_string(),
        ));
    }
    let pool = expect_context::<SqlitePool>();
    let todo = sqlx::query_as::<_, Todo>(
        "INSERT INTO todos (title, completed) VALUES (?, FALSE) RETURNING id, title, completed",
    )
    .bind(&title)
    .fetch_one(&pool)
    .await
    .map_err(db_err)?;
    Ok(todo)
}

#[server(UpdateTodo, "/api")]
pub async fn update_todo(id: i64, title: String, completed: bool) -> Result<(), ServerFnError> {
    use sqlx::SqlitePool;
    let title = title.trim().to_string();
    if title.is_empty() {
        // Empty title after edit means delete the todo
        return delete_todo(id).await;
    }
    let pool = expect_context::<SqlitePool>();
    sqlx::query("UPDATE todos SET title = ?, completed = ? WHERE id = ?")
        .bind(&title)
        .bind(completed)
        .bind(id)
        .execute(&pool)
        .await
        .map_err(db_err)?;
    Ok(())
}

#[server(DeleteTodo, "/api")]
pub async fn delete_todo(id: i64) -> Result<(), ServerFnError> {
    use sqlx::SqlitePool;
    let pool = expect_context::<SqlitePool>();
    sqlx::query("DELETE FROM todos WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(db_err)?;
    Ok(())
}

#[server(ToggleAll, "/api")]
pub async fn toggle_all(completed: bool) -> Result<(), ServerFnError> {
    use sqlx::SqlitePool;
    let pool = expect_context::<SqlitePool>();
    sqlx::query("UPDATE todos SET completed = ?")
        .bind(completed)
        .execute(&pool)
        .await
        .map_err(db_err)?;
    Ok(())
}

#[server(ClearCompleted, "/api")]
pub async fn clear_completed() -> Result<(), ServerFnError> {
    use sqlx::SqlitePool;
    let pool = expect_context::<SqlitePool>();
    sqlx::query("DELETE FROM todos WHERE completed = TRUE")
        .execute(&pool)
        .await
        .map_err(db_err)?;
    Ok(())
}
