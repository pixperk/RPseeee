use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType{
    Request = 0,
    Response = 1,
    Error = 2,
}

const DEFAULT_VERSION: u8 = 1;

#[derive(Serialize, Deserialize, Debug)]
pub struct  Message{
    version :  u8,
    message_type : MessageType,
    request_id : u64,
    method : String,
    payload : Vec<u8>
}

pub fn new_message(message_type : MessageType, request_id : u64, method: String, payload : &str) -> Message{
    let byte_payload = payload.as_bytes().to_vec();
    Message { version: DEFAULT_VERSION, message_type, request_id, method, payload: byte_payload }
}

impl Message{
    pub fn get_payload_as_str(&self) -> anyhow::Result<&str>{
        let payload_str = std::str::from_utf8(&self.payload)?;
        Ok(payload_str)
    }
}