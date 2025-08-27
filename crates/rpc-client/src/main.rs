use rpc_client::new_client;
use rpc_core:: message::{new_message, MessageType};

#[tokio::main]
pub async fn main() -> anyhow::Result<()>{
    let mut client = new_client("127.0.0.1:8080".to_owned());
    let sample_json_payload = r#"
        {
            "method": "echo",
            "params": ["Hello, daddy!"]
        }
    "#;
    let msg = new_message(MessageType::Request, 42069, "fuckmedaddy".to_owned(), sample_json_payload );

    client.connect_to_server().await?;
    println!("connected to server on addr : 127.0.0.1:8080");
    client.send_msg(msg).await?;
   
    Ok(())
}