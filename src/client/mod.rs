use log::*;
use url::Url;
use tungstenite::Message;
use tokio::net::TcpListener;
use std::io::{Error, ErrorKind};
use futures_util::{StreamExt, SinkExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Default)]
struct Properties {
    local: String,
    ws: String,
}

fn parse_args() -> Option<Properties> {
    let mut config = Properties::default();
    let mut args = std::env::args().skip(1);
    while let Some(arg)= args.next() {
        match arg.as_str() {
            "--local" | "-local" | "--l" | "-l" => {
                let local = match args.next() {
                    Some(it) => it,
                    _ => {
                        error!("请指定本地服务端口, 如: -l 1066 或 -l 127.0.0.1:1066");
                        return None;
                    }
                };
                config.local = local;
            },
            "--websocket" | "-websocket" | "--ws" | "-ws" | "-w" => {
                let ws = match args.next() {
                    Some(it) => {
                        if !it.starts_with("ws://") && !it.starts_with("wss://"){
                            format!("ws://{it}")
                        } else {
                            it
                        }
                    },
                    _ => "".to_string(),
                };
                config.ws = ws;
            },
            _ => {}
        }
    }
    if config.local.is_empty() {
        config.local = "127.0.0.1:1066".to_string();
    }
    if config.ws.is_empty() {
        error!("请指定 Websocket URL, 如: -w ws://localhost:8080");
        return None;
    }
    Some(config)
}

pub async fn run() -> Result<(), Error> {
    match parse_args() {
        Some(config) => server(&config).await,
        _ => Ok(())
    }
}

async fn friendly_bind(local: &str) -> Result<TcpListener, Error> {
    match TcpListener::bind(local).await {
        Ok(server) => Ok(server),
        Err(e) if e.kind() == ErrorKind::AddrInUse => {
            error!("地址 {} 已被占用: {}", local, e);
            Err(e)
        },
        Err(e) if e.kind() == ErrorKind::InvalidInput => {
            Err(e)
        },
        Err(e) => {
            error!("地址 {} 绑定失败: {}", local, e);
            Err(e)
        }
    }
}

async fn server(config: &Properties) -> Result<(), Error> {
    let server = match friendly_bind(&config.local).await {
        Ok(it) => it,
        Err(e) if e.kind() == ErrorKind::InvalidInput => { // 如果用户只提供了端口号
            match friendly_bind(&format!("127.0.0.1:{}", &config.local)).await {
                Ok(it) => it,
                Err(e) if e.kind() == ErrorKind::InvalidInput => { // 端口号不正确
                    error!("请提供正确的端口号!");
                    return Ok(());
                },
                _ => return Ok(())
            }
        },
        Err(_) => return Ok(()),
    };
    let local = server.local_addr().unwrap();
    info!("Service running on: {local}");
    let url = Url::parse(&config.ws).expect("WebSocket地址错误");
    info!("WebSocket Interface: {}", url);
    while let Ok((mut stream, peer)) = server.accept().await {
        info!("Client connected: {}", peer);
        info!("Connecting to {}", url.to_string());
        let (socket, response) = match tokio_tungstenite::connect_async(&url).await {
            Ok(conn) => conn,
            Err(e) => {
                error!("Failed to connect websocket, cause {}!", e);
                continue;
            }
        };
        let (mut write, mut read) = socket.split();
        info!("Connected to the server");
        info!("Response HTTP code: {}", response.status());
        debug!("Response contains the following headers:");
        for (ref header, value) in response.headers() {
            debug!("* {}: {:?}", header.as_str().to_uppercase(), value);
        }

        let mut buf = Vec::with_capacity(10240);
        tokio::spawn(async move {
            loop {
                debug!("loop...");
                buf.clear(); // reset buf length
                tokio::select! {
                    n = stream.read_buf(&mut buf) => {
                        match n {
                            Ok(0) => {
                                debug!("客户端主动shutdown连接");
                                break;
                            },
                            Ok(n) => {
                                debug!("读取TCP {} 字节", n);
                                let bin = Message::binary(&buf[..n]);
                                match write.send(bin).await {
                                    Ok(()) => {
                                        debug!("写入WS成功");
                                    }
                                    _ => {
                                        error!("写入WS失败!");
                                        break;
                                    }
                                }
                            },
                            Err(e) => {
                                debug!("读取客户端数据时发生了错误: {e}");
                                break;
                            },
                        }
                    },
                    msg = read.next() => {
                        match msg {
                            Some(msg) => {
                                match msg {
                                    Ok(Message::Binary(msg)) => {
                                        debug!("接收到ws二进制消息: {:?}", msg);
                                        let n = stream.write(&msg).await;
                                        debug!("n = {:?}", n);
                                    },
                                    Ok(Message::Text(msg)) => {
                                        debug!("接收到ws文本消息: {:?}", msg);
                                        // stream.write(msg.as_bytes()).await;
                                    },
                                    Ok(msg) => {
                                        debug!("接收到ws其它消息: {:?}", msg);
                                        break; // warning
                                    },
                                    Err(e) => {
                                        debug!("接收到ws错误: {:?}", e);
                                        break;
                                    },
                                }
                            }
                            None => {
                                debug!("接收到ws消息: None");
                                break;
                            }
                        }
                    }
                };
            }
        });
    }
    Ok(())
}