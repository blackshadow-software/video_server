use crate::rest::functions::download::{stream_file, stream_video};
use axum::{response::Html, routing::get, Router};
use tower_http::cors::{Any, CorsLayer};

pub async fn routes() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/video/{filename}", get(stream_video))
        .route("/file/{filename}", get(stream_file))
        .layer(CorsLayer::new().allow_origin(Any))
    // .route("/project", get(project))
    // .route("/images", get(images))
    // .route("/test_api", post(test_api))
    // .route("/signup", post(signup_api))
    // .route("/create_group", post(create_group_api))
    // .route("/my_groups", post(my_groups))
}
pub async fn root() -> Html<&'static str> {
    Html(
        r#"
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
  <title>Rust API Landing</title>
  <style>
    body {
      background-color: #1e1e1e;
      color: #ffffff;
      font-family: 'Comic Sans MS', cursive, sans-serif;
      text-align: center;
      padding-top: 5em;
      margin: 0;
    }

    h1 {
      font-size: 3em;
      margin-bottom: 0.2em;
    }

    p {
      font-size: 1.3em;
      margin-bottom: 2em;
    }

    .rust-logo {
      width: 100px;
      height: 100px;
      animation: spin 4s linear infinite;
      margin: 0 auto 2em;
      filter: brightness(0) invert(1); /* makes it white on dark bg */
    }

    @keyframes spin {
      0%   { transform: rotate(0deg); }
      100% { transform: rotate(360deg); }
    }

    .ascii {
      font-family: monospace;
      font-size: 0.9em;
      color: #ff7e00;
      margin-top: 2em;
    }

    .footer {
      margin-top: 4em;
      font-size: 0.8em;
      opacity: 0.6;
    }
    .ascii {
  font-family: monospace;
  font-size: 1em;
  color:rgb(255, 119, 0);
  margin-top: 2em;
  text-align: left;
  width: max-content;
  margin-left: auto;
  margin-right: auto;
  background-color:rgba(44, 44, 44, 0.59);
  padding: 0.5em;
  border-radius: 25px;
  box-shadow: 0 0 30px rgba(255, 0, 0, 0.32);
}

  </style>
</head>
<body>
  <h1>🦀 Welcome to the Rust API!</h1>
  <p>Zero-cost abstractions and 100% fun. Brace for <code>unsafe</code> levels of speed.</p>
  
  <img class="rust-logo" src="https://www.rust-lang.org/static/images/rust-logo-blk.svg" alt="Rust Logo" />

  <div class="ascii">
    <pre><code>
    fn main() {
        println!("Black Shadow Software");
    }
    </code></pre>
  </div>

  <div class="footer">
    This page was compiled in 0.0 seconds — probably. <br/>
    Try hitting <code>/api/v1/json</code> if you're looking for actual payloads.
  </div>
</body>
</html>
"#,
    )
}
