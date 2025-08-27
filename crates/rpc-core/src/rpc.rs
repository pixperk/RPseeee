use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcRequest {
    pub method: String,
    pub params: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcResponse {
    pub result: Result<serde_json::Value, RpcError>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RpcError {
    MethodNotFound(String),
    InvalidParams(String),
    InternalError(String),
    ParseError(String),
}

impl std::fmt::Display for RpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RpcError::MethodNotFound(method) => write!(f, "Method not found: {}", method),
            RpcError::InvalidParams(msg) => write!(f, "Invalid parameters: {}", msg),
            RpcError::InternalError(msg) => write!(f, "Internal error: {}", msg),
            RpcError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl std::error::Error for RpcError {}

impl RpcRequest {
    pub fn new(method: String, params: serde_json::Value) -> Self {
        Self { method, params }
    }
    
    pub fn get_string_param(&self, key: &str) -> Result<String, RpcError> {
        self.params
            .get(key)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| RpcError::InvalidParams(format!("Missing or invalid parameter: {}", key)))
    }
    
    pub fn get_raw_params(&self) -> String {
        match &self.params {
            serde_json::Value::String(s) => s.clone(),
            _ => self.params.to_string(),
        }
    }
}

impl RpcResponse {
    pub fn success(result: serde_json::Value) -> Self {
        Self {
            result: Ok(result),
        }
    }
    
    pub fn error(error: RpcError) -> Self {
        Self {
            result: Err(error),
        }
    }
}