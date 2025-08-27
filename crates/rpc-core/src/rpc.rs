pub struct RpcRequest{
    pub method : String,
    pub params : Vec<u8>
}

pub struct RpcResponse{
    pub result : Result<Vec<u8>, String>
}