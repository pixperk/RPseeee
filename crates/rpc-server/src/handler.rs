use rpc_core::rpc::{RpcRequest, RpcResponse};

#[async_trait::async_trait]
pub trait Handler : Send + Sync{
    async fn handle(&self, request: RpcRequest) -> RpcResponse;
}
