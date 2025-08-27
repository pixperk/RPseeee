use rpc_server::new_server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server = new_server("127.0.0.1:8080".to_owned());
    server.start().await
}


