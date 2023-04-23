use std::collections::HashMap;

use axum::*;
use futures_util::{SinkExt, StreamExt};
use io::{AsyncReadExt, AsyncWriteExt};
use once_cell::sync::Lazy;
use tokio::*;
use task::JoinHandle;
use sync::Mutex;
use tokio_tungstenite::tungstenite::Message;

use super::*;
use config::*;

async fn on_connect(url: &str, mut stream: net::TcpStream) {
    let request = http::Request::get(url).body(()).unwrap();
    // let to = request; // WebSocket protocol error: Missing, duplicated or incorrect header sec-websocket-key!
    let to = request.uri();
    let (ws, _response) = match tokio_tungstenite::connect_async(to).await {
        Ok(conn) => conn,
        Err(e) => {
            println!("Failed to connect websocket, cause {}!", e);
            return;
        }
    };
    let (mut write, mut read) = ws.split();
    let mut buf = Vec::with_capacity(10240);
    spawn(async move {
        loop {
            buf.clear(); // reset buf length
            select! {
                n = stream.read_buf(&mut buf) => {
                    match n {
                        Ok(0) => {
                            write.close().await.unwrap();
                            break;
                        },
                        Ok(n) => {
                            let bin = Message::binary(&buf[..n]);
                            match write.send(bin).await {
                                Ok(_) => (),
                                _ => break
                            }
                        },
                        _ => break
                    }
                },
                msg = read.next() => {
                    match msg {
                        Some(msg) => {
                            match msg {
                                Ok(Message::Binary(ref msg)) if stream.write(msg).await.is_ok() => (),
                                Ok(_) => continue,
                                _ => break
                            }
                        },
                        _ => break
                    }
                }
            };
        }
    });
}

static MAP: Lazy<Mutex<HashMap<Item, JoinHandle<()>>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

pub async fn start(item: Item) -> std::io::Result<()> {
    match net::TcpListener::bind(&item.listen).await {
        Ok(sock) => {
            let ws = item.ws.clone();
            let task = spawn(async move {
                loop {
                    let (stream, _) = sock.accept().await.unwrap();
                    on_connect(&ws, stream).await;
                }
            });
            let mut map = MAP.lock().await;
            map.insert(item, task);
            Ok(())
        },
        Err(ref e) if e.kind() == std::io::ErrorKind::AddrInUse => {
            Err(std::io::ErrorKind::AddrInUse.into())
        },
        Err(e) => {
            Err(e)
        },
    }
}

pub async fn stop(item: Item) -> std::io::Result<()> {
    let mut map = MAP.lock().await;
    match map.get(&item) {
        Some(task) => {
            task.abort();
            map.remove(&item);
            Ok(())
        },
        _ => Err(std::io::ErrorKind::NotFound.into()),
    }
}

pub async fn list() -> std::io::Result<Vec<Item>> {
    let map = MAP.lock().await;
    let items = map.iter().map(|(it, _)| {
        it.clone()
    }).collect::<Vec<_>>();
    Ok(items)
}