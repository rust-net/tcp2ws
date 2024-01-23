use std::collections::HashMap;

use axum::*;
use futures_util::{SinkExt, StreamExt};
use io::{AsyncReadExt, AsyncWriteExt};
use once_cell::sync::Lazy;
use tokio::{*, net::UdpSocket};
use task::JoinHandle;
use sync::Mutex;
use tokio_tungstenite::tungstenite::Message;

use super::*;
use config::*;

async fn on_connect(url: String, mut stream: net::TcpStream) {
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
}

static MAP: Lazy<Mutex<HashMap<Item, (JoinHandle<()>, JoinHandle<()>)>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

async fn server_udp(item: &Item) -> Result<JoinHandle<()>, Error> {
    match UdpSocket::bind(&item.listen).await {
        Ok(udp) => {
            let u = item.udp.to_string();
            let task = tokio::spawn(async move {
                loop {
                    let mut buf = vec![0u8; 1024];
                    let (le, who) = udp.recv_from(&mut buf).await.unwrap();
                    let c = UdpSocket::bind("0.0.0.0:0").await.unwrap();
                    c.connect(&u).await.unwrap();
                    c.send(&buf[..le]).await.unwrap();
                    let le = c.recv(&mut buf).await.unwrap();
                    udp.send_to(&buf[..le], who).await.unwrap();
                }
            });
            return Ok(task);
        }
        Err(e) => {
            macro_log::e!("{e}");
        }
    }
    Ok(tokio::spawn(async {}))
}

pub async fn start(item: Item) -> std::io::Result<()> {
    let udp_task = server_udp(&item).await.unwrap();
    match net::TcpListener::bind(&item.listen).await {
        Ok(sock) => {
            let ws = item.ws.clone();
            let task = spawn(async move {
                loop {
                    let (stream, _) = sock.accept().await.unwrap();
                    spawn(on_connect(ws.clone(), stream));
                }
            });
            let mut map = MAP.lock().await;
            map.insert(item, (task, udp_task));
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
        Some((tcp, udp)) => {
            tcp.abort();
            udp.abort();
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