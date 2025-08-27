use rpc_client::new_client;
use rpc_core:: message::{new_message, MessageType};
use std::io::{self, Write};

#[tokio::main]
pub async fn main() -> anyhow::Result<()>{
    let mut client = new_client("127.0.0.1:8080".to_owned());
    
    
    print!("Enter your name: ");
    io::stdout().flush()?; 
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let name = input.trim(); 
    
    let msg = new_message(MessageType::Request, 42069, "hello".to_owned(), name);

    client.connect_to_server().await?;
    println!("connected to server on addr : 127.0.0.1:8080");
    println!("requesting hello for: {}", name);
    let response = client.send_msg(msg).await?;
    println!("hello response : {:?}", response.get_payload_as_str()?);

    Ok(())
}