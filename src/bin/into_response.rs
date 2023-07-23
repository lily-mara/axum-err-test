use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use eyre::{eyre, Result};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler)).with_state(0i32);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Result<String, AppError> {
    Err(eyre!("oops").into())
}

struct AppError(eyre::Report);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!(
                "state={} - Something went wrong: {}",
                todo!("how do i access state??"),
                self.0
            ),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<eyre::Report>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
