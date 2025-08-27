use rpc_client::new_client;
use serde_json::json;

#[tokio::main]
pub async fn main() -> anyhow::Result<()>{
    let mut client = new_client("127.0.0.1:8080".to_owned());
    
    client.connect_to_server().await?;
    println!("Connected to server on 127.0.0.1:8080");
    println!();

    println!("Test 1: Simple hello with string");

    let name = "World";

    let response = client.call_simple("hello", name).await?;
    println!("Response: {}", response);
    println!();

    println!("Test 2: Hello with JSON object");
    let response = client.call_json("hello", json!({"name": name})).await?;
    println!("Response: {}", response);
    println!();

    println!("Test 3: Echo with string");
    let message = "This is a test message.";
    let response = client.call_simple("echo", message).await?;
    println!("Response: {}", response);
    println!();

    println!("Test 4: Echo with JSON object");
    let response = client.call_json("echo", json!({"message": message})).await?;
    println!("Response: {}", response);
    println!();

    println!("\nAll tests completed!");
    Ok(())
}