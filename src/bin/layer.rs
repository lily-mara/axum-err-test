use axum::{
    error_handling::HandleErrorLayer, extract::State, http::StatusCode, routing::get, BoxError,
    Router,
};
use tower::ServiceBuilder;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler)) // Does not compile because eyre::Report can't be used as return type
        .with_state(0i32)
        .layer(ServiceBuilder::new().layer(HandleErrorLayer::new(handle_error))); // Does not compile

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_error(State(state): State<i32>, error: BoxError) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("state={} - Something went wrong: {}", state, error,),
    )
}

async fn handler() -> Result<String, eyre::Report> {
    eyre::bail!("oops")
}
