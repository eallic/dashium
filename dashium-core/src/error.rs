use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[allow(unused)]
pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        #[cfg(debug_assertions)]
        {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong: {}", self.0),
            )
                .into_response()
        }

        #[cfg(not(debug_assertions))]
        {
            (StatusCode::INTERNAL_SERVER_ERROR, "-1".to_string()).into_response()
        }
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
