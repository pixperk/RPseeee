use std::sync::Arc;

use rpc_core::framing::{read_frame, write_frame};
use tokio::net::{TcpListener, TcpStream};

use crate::registry::Registry;

pub mod registry;
pub mod handler;

pub struct Server {
    listen_addr: String,
    registry : Registry,
}

pub fn new_server(listen_addr: String) -> Server {
    Server { 
        listen_addr,
        registry: Registry::new(),
    }
}

impl Server {
    pub fn register_handler(&mut self, method: String, handler: Arc<dyn crate::handler::Handler>) {
        self.registry.register_handler(method, handler);
    }

    pub async fn start(&self) -> anyhow::Result<()> {
        let lis = TcpListener::bind(&self.listen_addr).await?;
        println!("server running on {}", self.listen_addr);
        Server::listen_and_accept(self, lis).await

    }

    async fn listen_and_accept(&self, lis: TcpListener) -> anyhow::Result<()> {
        loop {
            let (mut socket, addr) = lis.accept().await?;
            println!("new connection from {:?}", addr);

        
            let registry = self.registry.clone();
            tokio::spawn(async move {
                if let Err(e) = Server::handle_client(registry, &mut socket).await {
                    println!("error handling client: {:?}", e);
                }
            });
        }
    }

    async fn handle_client(registry: Registry, stream: &mut TcpStream) -> anyhow::Result<()> {
        loop {
            let req = read_frame(stream).await?;
            let req_method = req.get_method();
            if let Some(handler) = registry.get_handler(req_method){
                let payload = req.get_payload_as_str()?;
                let response_payload = handler.handle(payload.as_bytes()).await;
                
                let response = rpc_core::message::new_message(
                    rpc_core::message::MessageType::Response,
                    req.get_request_id(),
                    req_method.to_string(),
                    &String::from_utf8_lossy(&response_payload)
                );
                
                write_frame(stream, &response).await?;
            } else {
                let error_response = rpc_core::message::new_message(
                    rpc_core::message::MessageType::Error,
                    req.get_request_id(),
                    req_method.to_string(),
                    "Method not found"
                );
                write_frame(stream, &error_response).await?;
            }
        }
    }
}
