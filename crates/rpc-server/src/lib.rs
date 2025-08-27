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
            let message = read_frame(stream).await?;
            let method = message.get_method();
            
            if let Some(handler) = registry.get_handler(method) {
                let rpc_request = match message.get_payload_as_str() {
                    Ok(payload_str) => {
                        // Try to parse payload as JSON first
                        match serde_json::from_str::<serde_json::Value>(payload_str) {
                            Ok(json_value) => {
                                // If it's valid JSON, use it as params
                                rpc_core::rpc::RpcRequest::new(
                                    method.to_string(),
                                    json_value
                                )
                            }
                            Err(_) => {
                                // If not valid JSON, treat as simple string
                                rpc_core::rpc::RpcRequest::new(
                                    method.to_string(),
                                    serde_json::Value::String(payload_str.to_string())
                                )
                            }
                        }
                    }
                    Err(_) => {
                        let error_response = rpc_core::message::new_message(
                            rpc_core::message::MessageType::Error,
                            message.get_request_id(),
                            method.to_string(),
                            "Invalid payload encoding"
                        );
                        write_frame(stream, &error_response).await?;
                        continue;
                    }
                };
                
                // Call handler with RpcRequest
                let rpc_response = handler.handle(rpc_request).await;
                
                // Convert RpcResponse back to message
                let (message_type, payload) = match rpc_response.result {
                    Ok(result) => (
                        rpc_core::message::MessageType::Response,
                        result.to_string()
                    ),
                    Err(error) => (
                        rpc_core::message::MessageType::Error,
                        error.to_string()
                    ),
                };
                
                let response_message = rpc_core::message::new_message(
                    message_type,
                    message.get_request_id(),
                    method.to_string(),
                    &payload
                );
                
                write_frame(stream, &response_message).await?;
            } else {
                // Method not found
                let error_response = rpc_core::message::new_message(
                    rpc_core::message::MessageType::Error,
                    message.get_request_id(),
                    method.to_string(),
                    "Method not found"
                );
                write_frame(stream, &error_response).await?;
            }
        }
    }
}
