# RPC Library

A simple async Rust RPC library built with Tokio.

## Features

- Async/await support with Tokio
- JSON-based message protocol
- Binary message framing
- Concurrent client handling
- Type-safe handler system

## Architecture

```
rpc_lib/
├── crates/
│   ├── rpc-core/     # Core types and protocol
│   ├── rpc-server/   # Server implementation
│   └── rpc-client/   # Client implementation
```

## Usage

### Server

```rust
use rpc_server::{handler::Handler, new_server};
use rpc_core::rpc::{RpcRequest, RpcResponse};
use std::sync::Arc;

struct HelloHandler;

#[async_trait::async_trait]
impl Handler for HelloHandler {
    async fn handle(&self, request: RpcRequest) -> RpcResponse {
        let name = request.get_raw_params();
        RpcResponse::success(serde_json::Value::String(format!("Hello, {}!", name.trim())))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut server = new_server("127.0.0.1:8080".to_owned());
    server.register_handler("hello".to_string(), Arc::new(HelloHandler));
    server.start().await
}
```

### Client

```rust
use rpc_client::new_client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = new_client("127.0.0.1:8080".to_owned());
    client.connect_to_server().await?;
    
    let response = client.call_simple("hello", "World").await?;
    println!("Response: {}", response);
    
    Ok(())
}
```

## Running

Start the server:
```bash
cd crates/rpc-server && cargo run
```

Run the client:
```bash
cd crates/rpc-client && cargo run
```

## Future Goals

- Middleware support for request/response processing
- Streaming support for large data transfers
- Distributed system features (service discovery, load balancing)
- Better error handling and retries
- Performance optimizations

## License

MIT
