use axum::{response::{IntoResponse, Response}, http::StatusCode};

#[derive(Debug)]
pub struct AppError(anyhow::Error);

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(e: E) -> Self {
        Self(e.into())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR, 
            format!("something went wrong: {}", self.0)
        ).into_response()
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
