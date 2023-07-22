use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use sqlx::{FromRow, PgPool};
use crate::error::Result;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

pub async fn get_user(name: String, pool: PgPool) -> Result<User> {
    let user = sqlx::query_as!(User, "select * from users where name=$1", name)
            .fetch_one(&pool)
            .await?;

    Ok(user)           
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_get_user(pool: PgPool) {
        let user = super::get_user("admin".to_string(), pool).await.unwrap();
        assert_eq!(user.id, 1i32);
        assert_eq!(user.name, "admin".to_string());
        assert_eq!(user.email, "admin@localhost".to_string());
    }
}
