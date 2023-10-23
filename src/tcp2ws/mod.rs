mod route;
mod api;
mod config;
mod service;

use config::Config;

use crate::proc::config::get_config;

// #[tokio::main(flavor = "multi_thread", worker_threads = 5)]
#[tokio::main(flavor = "current_thread")]
async fn entry(socket: std::net::TcpListener, config: Config) {
    let server = axum::Server::from_tcp(socket).unwrap()
        .serve(route::router(config).into_make_service_with_connect_info::<std::net::SocketAddr>());
    println!("server run on: {}", server.local_addr());
    server.await.unwrap();
}

#[cfg(windows)]
#[allow(unused)]
extern "system" {
    fn FreeConsole();
    fn AllocConsole();
    fn WinExec(cmd: *const u8, cmdShow: u8);
}

fn start() {
    let config = std::fs::read(get_config()).unwrap_or(br#"{ "port": 6601, "item": [] }"#.to_vec());
    let config: Config = serde_json::from_str(&String::from_utf8_lossy(&config).to_string()).unwrap();
    #[cfg(all(windows, not(debug_assertions)))]
    unsafe {
        WinExec(format!("explorer http://127.0.0.1:{}\0", config.port).as_ptr(), 0);
        FreeConsole();
        // AllocConsole();
    }
    #[cfg(unix)]
    unsafe {
        libc::setsid();
        std::process::Command::new("open")
            .args([format!("http://127.0.0.1:{}", config.port)])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn().unwrap();
    }
    match std::net::TcpListener::bind(format!("0.0.0.0:{}", config.port)) {
        Ok(socket) => {
            entry(socket, config);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}

pub fn main() {
    #[cfg(any(windows, unix))]
    match std::env::var("detatch") {
        Ok(v) if v == "1" => {
            start();
        }
        _ => {
            let exe = std::env::current_exe().unwrap();
            let mut process = std::process::Command::new(exe);
            process.arg("detatch");
            process.env("detatch", "1");
            process.stdout(std::process::Stdio::null());
            process.stderr(std::process::Stdio::null());
            process.spawn().unwrap();
            return;
        },
    }
}