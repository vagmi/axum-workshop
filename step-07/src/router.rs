mod todos;

use axum::{Router, routing::get, extract::Path};
use tower_http::trace::TraceLayer;

use crate::{greeting::greet, state::AppState};

async fn hello_name(Path(name): Path<String>) -> String{
    greet(&name)
}

async fn hello_world() -> String {
    greet("tokio world")
}

pub fn build_router(app_state: AppState) -> Router {
    Router::new()
            .route("/", get(hello_world))
           .route("/greet/:name", get(hello_name))
           .nest("/api/todos", todos::build_router())
           .layer(TraceLayer::new_for_http())
           .with_state(app_state)
           
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{http::Request, body::Body};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_hello_world() {
        assert_eq!(hello_world().await, "Hello, tokio world!");
    }

    #[tokio::test]
    async fn test_hello_name() {
        assert_eq!(hello_name(Path("tokio".to_string())).await, "Hello, tokio!");
    }

    #[tokio::test]
    async fn test_build_router() {
        let state = AppState::new().await.unwrap();
        let router = build_router(state);
        let resp = router.oneshot(Request::builder().uri("/")
                         .body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(resp.status(), 200);
        let content = resp.into_body();
        let resp_body = hyper::body::to_bytes(content).await.unwrap();
        assert_eq!(resp_body, "Hello, tokio world!".as_bytes());
    }
}
