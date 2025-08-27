use rpc_server::{handler::Handler, new_server};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]


struct HelloHandler;

#[async_trait::async_trait]
impl Handler for HelloHandler{
    async fn handle(&self, payload : &[u8]) -> Vec<u8>{
        //convert payload to string
        let request_str = String::from_utf8_lossy(payload);
        //create response
        let response = format!("Hello, {}!", request_str);
        response.into_bytes()
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut server = new_server("127.0.0.1:8080".to_owned());
    
    // Register the weather handler
    server.register_handler("hello".to_string(), Arc::new(HelloHandler));
    
    println!("Hello RPC Server starting on 127.0.0.1:8080");
    println!("Send requests with method 'hello' and your name as payload");
    
    server.start().await
}


