//! main.rs
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8000")
        .expect("Failed to bind");

    z2p::run(listener)?.await
}
