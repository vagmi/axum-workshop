use axum::{Router, Json, extract::{State, Path}, routing::{get, post}};
use crate::state::{Todo, AppState};

#[axum_macros::debug_handler]
pub async fn list_todos(State(app_state): State<AppState>) -> Json<Vec<Todo>> {
    let todos = app_state.todos.lock().unwrap();
    Json(todos.clone())
}

pub async fn create_todo(State(app_state): State<AppState>, Json(todo): Json<Todo>) -> Json<Todo> {
    let mut todos = app_state.todos.lock().unwrap();
    todos.push(todo.clone());
    Json(todo)
}

pub async fn toggle_todo(Path(id): Path<u64>, State(app_state): State<AppState>) -> Json<Option<Todo>> {
    let mut todos = app_state.todos.lock().unwrap();
    let todo = todos.iter_mut().find(|todo| todo.id == id);
    if let Some(todo) = todo {
        todo.completed = !todo.completed;
        return Json(Some(todo.clone()));
    } 
    Json(None)
}

pub fn build_router() -> Router<AppState> {
    Router::new()
            .route("/", get(list_todos))
            .route("/", post(create_todo))
            .route("/:id/toggle", post(toggle_todo))
}
