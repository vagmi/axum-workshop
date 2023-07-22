use axum::{Router, Json, extract::{State, Path}, routing::{get, post}};
use crate::{state::AppState, db::{Todo, CreateTodo}};
use crate::auth::Auth;
use crate::error::Result;
use crate::db as db;

#[axum_macros::debug_handler]
pub async fn list_todos(
    State(app_state): State<AppState>,
    Auth(user): Auth
) -> Result<Json<Vec<Todo>>> {
    tracing::info!("user: {:?}", user);
    let todos = db::list_todos(app_state.pool.clone()).await?;
    Ok(Json(todos))
}

#[axum_macros::debug_handler]
pub async fn create_todo(
    State(app_state): State<AppState>, 
    Json(todo): Json<CreateTodo>
    ) -> Result<Json<Todo>> {
    let todo = db::create_todo(app_state.pool.clone(), todo).await?;
    Ok(Json(todo))
}

pub async fn toggle_todo(
    Path(id): Path<i32>, 
    State(app_state): State<AppState>
) -> Result<Json<Option<Todo>>> {
    let todo = db::complete_todo(app_state.pool.clone(), id).await?;
    Ok(Json(Some(todo)))
}

pub fn build_router() -> Router<AppState> {
    Router::new()
            .route("/", get(list_todos))
            .route("/", post(create_todo))
            .route("/:id/toggle", post(toggle_todo))
}
