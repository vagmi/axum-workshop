mod greeting;
mod router;

#[tokio::main]
async fn main() {
    let router = router::build_router();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let app = axum::serve(listener, router.into_make_service());
    println!("Listening on port 3000");
    app.await.unwrap();
}
