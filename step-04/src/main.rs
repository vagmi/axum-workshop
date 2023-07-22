mod greeting;
mod router;
mod state;

use axum::Server;

use crate::state::AppState;


#[tokio::main]
async fn main() {
    let router = router::build_router(AppState::default());
    let app = Server::bind(&"0.0.0.0:3000".parse().unwrap())
                .serve(router.into_make_service());
    println!("Listening on port 3000");
    app.await.unwrap();
}
