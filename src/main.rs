use std::io::Error;
use std::io::Write;
mod client;
mod server;

async fn run() -> Result<(), Error> {
    let mode = std::env::args().nth(1).unwrap_or_default();
    match mode.as_str() {
        "server" => server::run().await,
        _ => client::run().await,
    }
}

fn set_logger_env() {
    #[cfg(not(debug_assertions))]
    std::env::set_var("RUST_LOG", "info");
    #[cfg(debug_assertions)]
    std::env::set_var("RUST_LOG", "debug");
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    set_logger_env();
    // env_logger::init();
    env_logger::Builder::new()
        .parse_env("RUST_LOG")
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(), // 颜色丢失
                record.args()
            )
        })
        .init();
    run().await
}
