use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};
use bincode;
use crate::message::Message;

pub async  fn read_frame(stream : &mut TcpStream) -> bincode::Result<Message>{

    let mut length_bytes = [0u8; 4];
    stream.read_exact(&mut length_bytes).await?;

    let msg_len = u32::from_be_bytes(length_bytes) as usize;
    let mut buffer = vec![0u8; msg_len];
    stream.read_exact(&mut buffer).await?;
    
    let message : Message = bincode::deserialize(&buffer)?;
    Ok(message)
}

pub async fn write_frame(stream : &mut TcpStream, msg :  &Message)-> bincode::Result<()>{
    let msg_bytes = bincode::serialize(&msg)?;
    let len = msg_bytes.len() as u32;
    let len_bytes = len.to_be_bytes();

    stream.write_all(&len_bytes).await?;
    stream.write_all(&msg_bytes).await?;

    stream.flush().await?;
    Ok(())
}