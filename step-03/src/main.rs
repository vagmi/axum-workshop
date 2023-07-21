mod greeting;
mod router;

use axum::Server;


#[tokio::main]
async fn main() {
    let router = router::build_router();
    let app = Server::bind(&"0.0.0.0:3000".parse().unwrap())
                .serve(router.into_make_service());
    println!("Listening on port 3000");
    app.await.unwrap();
}
