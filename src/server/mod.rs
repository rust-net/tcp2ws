use std::{env, io::Error};
use futures_util::{future, StreamExt, TryStreamExt};
use tokio::net::{TcpListener, TcpStream};

pub async fn run() -> Result<(), Error> {
    let addr = env::args().nth(2).unwrap_or_else(|| "127.0.0.1:8080".to_string());

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream));
    }

    Ok(())
}

async fn accept_connection(stream: TcpStream) {
    let addr = stream.peer_addr().expect("connected streams should have a peer address");
    let ip = addr.ip();
    println!("ip: {ip}");

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    let (write, read) = ws_stream.split();
    // We should not forward messages other than text or binary.
    read.try_filter(|msg| future::ready(msg.is_text() || msg.is_binary()))
        .forward(write)
        .await
        .expect("Failed to forward messages")
}