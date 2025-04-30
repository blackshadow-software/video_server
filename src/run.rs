use crate::rest::run::run_rest;

pub fn run() {
    // _ = thread::spawn(move || {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        match run_rest().await {
            Ok(_) => println!("ok"),
            Err(e) => println!("error: {:?}", e),
        }
    });
    // });
}
