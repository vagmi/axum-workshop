#[derive(Clone, Debug)]
pub struct AppState {
    pub pool: sqlx::PgPool,
    pub signing_key: String,
}

impl AppState {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let conn_str = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let signing_key = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let pool = sqlx::PgPool::connect(&conn_str)
            .await?;
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await?;
        Ok(Self { pool, signing_key })
    }
}
