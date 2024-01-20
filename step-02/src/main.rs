mod greeting;

use greeting::greet;
use axum::{Router, routing::get};

async fn hello_world() -> String {
    greet("tokio world")
}

#[tokio::main]
async fn main() {
    let router = Router::new()
                .route("/", get(hello_world));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let app= axum::serve(listener, router.into_make_service());
    println!("Listening on port 3000");
    app.await.unwrap();
}
