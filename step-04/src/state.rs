use std::sync::{Mutex, Arc};
use serde::{Serialize, Deserialize};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub completed: bool,
}

#[derive(Clone, Default, Debug)]
pub struct AppState {
    pub todos: Arc<Mutex<Vec<Todo>>>,
}
