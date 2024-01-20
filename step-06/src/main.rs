mod greeting;
mod router;
mod state;
mod db;
mod error;


use crate::state::AppState;


#[tokio::main]
async fn main() {
    let app_state = AppState::new().await.unwrap();
    let router = router::build_router(app_state);
    let listener = tokio::net::TcpListener::bind(&"0.0.0.0:3000").await.unwrap();
    let app = axum::serve(listener, router.into_make_service());
    println!("Listening on port 3000");
    app.await.unwrap();
}
