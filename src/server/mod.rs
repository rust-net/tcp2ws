use log::*;
use tungstenite::Message;
use futures_util::{StreamExt, SinkExt};
use std::{io::{Error, ErrorKind}, net::ToSocketAddrs};
use tokio::{net::{TcpListener, TcpStream}, io::{AsyncReadExt, AsyncWriteExt}};


#[derive(Default)]
struct Properties {
    local: String,
    proxy: String,
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
                        error!("请指定WebSocket服务端口, 如: -l 8080 或 -l 0.0.0.0:8080");
                        return None;
                    }
                };
                config.local = local;
            },
            "--proxy" | "-proxy" | "--p" | "-p" => {
                match args.next() {
                    Some(it) => {
                        match it.to_socket_addrs() {
                            Ok(mut addrs) => {
                                if let Some(_addr) = addrs.next() { // 解析成功
                                    config.proxy = it;
                                } else {
                                    error!("代理地址解析异常");
                                    return None;
                                }
                            }
                            Err(e) if e.kind() == ErrorKind::InvalidInput => { // 地址无效
                                let proxy = format!("127.0.0.1:{}", it);
                                if let Ok(_addrs) = proxy.to_socket_addrs() {
                                    config.proxy = proxy;
                                } else {
                                    error!("端口无效: {}", proxy);
                                    return None;
                                }
                            }
                            Err(e) => { // 解析失败
                                error!("代理地址解析失败: {}", e);
                                return None;
                            }
                        }
                    },
                    _ => {}
                };
            },
            _ => {}
        }
    }
    if config.local.is_empty() {
        config.local = "0.0.0.0:8080".to_string();
    }
    if config.proxy.is_empty() {
        error!("请指定要代理的服务地址, 如: -p 22 或 -p localhost:22");
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
            match friendly_bind(&format!("0.0.0.0:{}", &config.local)).await {
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
    info!("Proxy to: {}", config.proxy);

    while let Ok((stream, peer)) = server.accept().await {
        let dest = config.proxy.clone();
        tokio::spawn(async move {
            info!("Client connected: {}", peer);
            let tcp = match TcpStream::connect(dest).await {
                Ok(tcp) => tcp,
                Err(e) => {
                    error!("Client rejected: {} cause {}", peer, e);
                    return;
                }
            };
            accept_connection(stream, tcp).await;
        });
    }

    Ok(())
}

async fn accept_connection(ws: TcpStream, mut tcp: TcpStream) {
    let ws_stream = tokio_tungstenite::accept_async(ws)
        .await
        .expect("Error during the websocket handshake occurred");

    let (mut write, mut read) = ws_stream.split();

    loop {
        let mut buf = Vec::with_capacity(10240);
        tokio::select! {
            n = tcp.read_buf(&mut buf) => {
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
                                let n = tcp.write(&msg).await;
                                debug!("n = {:?}", n);
                            },
                            Ok(Message::Text(msg)) => {
                                debug!("接收到ws文本消息: {:?}", msg);
                                // tcp.write(msg.as_bytes()).await;
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
        }
    }
}