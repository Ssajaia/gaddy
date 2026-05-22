use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::handler::{Request, handle};

pub async fn run(addr: &str) {
    let listener = TcpListener::bind(addr).await
        .unwrap_or_else(|e| panic!("failed to bind {addr}: {e}"));

    println!("gaddy listening on http://{addr}");

    loop {
        match listener.accept().await {
            Ok((mut socket, addr)) => {
                println!("[{addr}] connected");

                tokio::spawn(async move {
                    let mut buf = vec![0u8; 4096];

                    let n = match socket.read(&mut buf).await {
                        Ok(0) | Err(_) => return, // connection closed or error
                        Ok(n) => n,
                    };

                    let raw = String::from_utf8_lossy(&buf[..n]);

                    let response = match Request::parse(&raw) {
                        Some(req) => {
                            println!("[{addr}] {} {}", req.method, req.path);
                            handle(req)
                        }
                        None => {
                            eprintln!("[{addr}] failed to parse request");
                            crate::response::Response::not_found()
                        }
                    };

                    if let Err(e) = socket.write_all(&response.into_bytes()).await {
                        eprintln!("[{addr}] write error: {e}");
                    }
                });
            }
            Err(e) => eprintln!("accept error: {e}"),
        }
    }
}
