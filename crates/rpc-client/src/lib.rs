use anyhow::Ok;
use tokio::net::TcpStream;
use serde_json::Value;

use rpc_core::{framing::{read_frame, write_frame}, message::Message};

pub struct Client{
    srv_addr : String,
    stream : Option<TcpStream>,
    next_request_id: u64,
}

pub fn new_client(srv_addr : String) -> Client{
    Client{
        srv_addr,
        stream : None,
        next_request_id: 1,
    }
}

impl Client{
    pub async fn connect_to_server(&mut self) -> anyhow::Result<()>{
        let stream = TcpStream::connect(&self.srv_addr).await?;
        self.stream = Some(stream);
        Ok(())
    }
    pub async fn send_msg(&mut self, msg : Message) -> anyhow::Result<Message> {
        let stream = self.stream.as_mut().ok_or_else(|| anyhow::anyhow!("Not connected to server"))?;
        write_frame(stream, &msg).await?;
        let received_msg = read_frame(stream).await?;
        Ok(received_msg)
    }

    pub async fn call_json(&mut self, method: &str, params: Value) -> anyhow::Result<String> {
        let request_id = self.next_request_id;
        self.next_request_id += 1;

        // Serialize params to JSON string
        let params_str = params.to_string();
        
        let msg = rpc_core::message::new_message(
            rpc_core::message::MessageType::Request,
            request_id,
            method.to_string(),
            &params_str
        );

        let response = self.send_msg(msg).await?;
        let result = response.get_payload_as_str()?;
        Ok(result.to_string())
    }

    
    pub async fn call_simple(&mut self, method: &str, param: &str) -> anyhow::Result<String> {
        self.call_json(method, Value::String(param.to_string())).await
    }

  
    pub async fn call_with_object(&mut self, method: &str, params: serde_json::Map<String, Value>) -> anyhow::Result<String> {
        self.call_json(method, Value::Object(params)).await
    }
}