use rpc_server::{handler::Handler, new_server};
use rpc_core::rpc::{RpcRequest, RpcResponse};
use std::sync::Arc;

struct HelloHandler;

#[async_trait::async_trait]
impl Handler for HelloHandler {
    async fn handle(&self, request: RpcRequest) -> RpcResponse {
        let name = request.get_raw_params();
        let response = format!("Hello, {}!", name.trim());
        
        RpcResponse::success(serde_json::Value::String(response))
    }
}

struct EchoHandler;

#[async_trait::async_trait]
impl Handler for EchoHandler {
    async fn handle(&self, request: RpcRequest) -> RpcResponse {
        let message = request.get_raw_params();
        let response = format!("Echo: {}", message);
        
        RpcResponse::success(serde_json::Value::String(response))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut server = new_server("127.0.0.1:8080".to_owned());

    
    // Register handlers
    server.register_handler("hello".to_string(), Arc::new(HelloHandler));
    server.register_handler("echo".to_string(), Arc::new(EchoHandler));
    
    println!("RPC Server starting on 127.0.0.1:8080");
    println!("Available methods: hello, echo");

    server.start().await
}


