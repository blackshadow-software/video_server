use crate::rest::functions::download::stream_video;
use axum::{routing::get, Router};
use tower_http::cors::{Any, CorsLayer};

pub async fn routes() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/video/{filename}", get(stream_video))
        .layer(CorsLayer::new().allow_origin(Any))
    // .route("/project", get(project))
    // .route("/images", get(images))
    // .route("/test_api", post(test_api))
    // .route("/signup", post(signup_api))
    // .route("/create_group", post(create_group_api))
    // .route("/my_groups", post(my_groups))
}

pub async fn root() -> &'static str {
    "Hello, World!"
}
