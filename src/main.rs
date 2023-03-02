use std::io::Error;
mod client;
mod server;

async fn run() -> Result<(), Error> {
    let mode = std::env::args().nth(1).unwrap_or_default();
    match mode.as_str() {
        "server" => server::run().await,
        _ => client::run().await,
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run().await
}
