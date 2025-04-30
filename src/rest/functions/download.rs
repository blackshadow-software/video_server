use std::path::PathBuf;

use axum::{
    body::Body,
    extract::Path,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use mime_guess::from_path;
use tokio::fs::File;
use tokio_util::io::ReaderStream;

use crate::utils::constraints::VIDEOS_SOURCE;

pub async fn stream_video(Path(filename): Path<String>) -> Result<Response, StatusCode> {
    let current_dir = std::env::current_dir().unwrap();
    println!("Current directory: {:?}", current_dir);

    let path = PathBuf::from(VIDEOS_SOURCE).join(&filename);
    if !path.exists() {
        println!("File not found: {:?}", path);
        return Err(StatusCode::NOT_FOUND);
    }
    println!("Streaming video from: {:?}", path);

    let mime_type = from_path(&path).first_or_octet_stream();

    match File::open(&path).await {
        Ok(file) => {
            let stream = ReaderStream::new(file);
            let body = Body::from_stream(stream);

            let mut headers = HeaderMap::new();
            headers.insert("Content-Type", mime_type.as_ref().parse().unwrap());

            Ok((headers, body).into_response())
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
