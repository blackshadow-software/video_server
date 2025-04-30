use std::thread;

use grpc_video_server::run_grpc_video_server;

use crate::{rest::run::run_rest, utils::constraints::VIDEOS_SOURCE};

pub fn run() {
    _ = thread::spawn(move || {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            match run_rest().await {
                Ok(_) => println!("ok"),
                Err(e) => println!("error: {:?}", e),
            }
        });
    });
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        match run_grpc_video_server(VIDEOS_SOURCE, "50051").await {
            Ok(_) => println!("ok"),
            Err(e) => println!("error: {:?}", e),
        };
    });
}
