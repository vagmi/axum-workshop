mod greeting;
mod router;
mod state;


use crate::state::AppState;


#[tokio::main]
async fn main() {
    let router = router::build_router(AppState::default());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let app = axum::serve(listener, router.into_make_service());
    println!("Listening on port 3000");
    app.await.unwrap();
}
