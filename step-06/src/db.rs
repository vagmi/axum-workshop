use serde::{Serialize, Deserialize};
use anyhow::Result;
use sqlx::{PgPool, FromRow};

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: Option<bool>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

pub async fn list_todos(pool: PgPool) -> Result<Vec<Todo>> {
    let todos = sqlx::query_as!(Todo, "SELECT * FROM todos")
                     .fetch_all(&pool).await?;
    Ok(todos)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTodo {
    pub title: String,
}
pub async fn create_todo(
    pool: PgPool,
    create_todo: CreateTodo,
) -> Result<Todo> {
    let todo = sqlx::query_as!(Todo, "INSERT INTO todos (title) VALUES ($1) RETURNING *", create_todo.title)
                     .fetch_one(&pool).await?;
    Ok(todo)
}

pub async fn complete_todo(pool: PgPool, id: i32) -> Result<Todo> {
    let todo = sqlx::query_as!(Todo, "UPDATE todos SET completed = true WHERE id = $1 RETURNING *", id)
                     .fetch_one(&pool).await?;
    Ok(todo)
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_list_todos(pool: PgPool) {
        let todos = super::list_todos(pool).await.unwrap();
        assert_eq!(todos.len(), 0);
    }

    #[sqlx::test]
    async fn test_create_todo(pool: PgPool) {
        let todo = super::create_todo(pool, super::CreateTodo{
            title: "create todo".to_string(),
        }).await.unwrap();
        
        assert_eq!(todo.id, 1);
        assert_eq!(todo.title, "create todo");
        assert_eq!(todo.completed, Some(false));
    }

    #[sqlx::test]
    async fn test_complete_todo(pool: PgPool) {
        let todo = super::create_todo(pool.clone(), super::CreateTodo{
            title: "complete todo".to_string(),
        }).await.unwrap();

        let completed_todo = super::complete_todo(pool, todo.id).await.unwrap();
        assert_eq!(todo.id, completed_todo.id);
        assert_eq!(todo.title, completed_todo.title);
        assert_eq!(completed_todo.completed, Some(true));
    }
}
