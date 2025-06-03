// LOCAL ADDITION: TCP proxy for external miners
use std::path::Path;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream, UnixStream};
use tracing::{error, info, warn};

pub async fn run_tcp_proxy(tcp_addr: String, unix_socket_path: String) -> std::io::Result<()> {
    let listener = TcpListener::bind(&tcp_addr).await?;
    info!("TCP proxy listening on {}", tcp_addr);

    loop {
        match listener.accept().await {
            Ok((tcp_stream, addr)) => {
                info!("New TCP connection from {}", addr);
                let unix_path = unix_socket_path.clone();
                tokio::spawn(async move {
                    if let Err(e) = handle_tcp_connection(tcp_stream, &unix_path).await {
                        error!("Error handling TCP connection: {}", e);
                    }
                });
            }
            Err(e) => {
                error!("Failed to accept TCP connection: {}", e);
            }
        }
    }
}

async fn handle_tcp_connection(
    tcp_stream: TcpStream,
    unix_socket_path: &str,
) -> std::io::Result<()> {
    let unix_stream = UnixStream::connect(Path::new(unix_socket_path)).await?;

    let (mut tcp_read, mut tcp_write) = tcp_stream.into_split();
    let (mut unix_read, mut unix_write) = unix_stream.into_split();

    // Simple bidirectional proxy
    tokio::select! {
        _ = tokio::io::copy(&mut tcp_read, &mut unix_write) => {},
        _ = tokio::io::copy(&mut unix_read, &mut tcp_write) => {},
    }

    Ok(())
}
