mod handler;
mod response;
mod server;

#[tokio::main]
async fn main() {
    server::run("0.0.0.0:7878").await;
}
