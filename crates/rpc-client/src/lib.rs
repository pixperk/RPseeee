use anyhow::Ok;
use tokio::net::TcpStream;

use rpc_core::{framing::{read_frame, write_frame}, message::Message};

pub struct Client{
    srv_addr : String,
    stream : Option<TcpStream>,
    //id : u64 //for now, change later
}

pub fn new_client(srv_addr : String) -> Client{
    Client{
        srv_addr,
        stream : None,
       /*  id: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(), */
    }
}

impl Client{
    pub async fn connect_to_server(&mut self) -> anyhow::Result<()>{
        let stream = TcpStream::connect(&self.srv_addr).await?;
        self.stream = Some(stream);
        Ok(())
    }
    pub async fn send_msg(&mut self, msg : Message) -> anyhow::Result<()> {
        let stream = self.stream.as_mut().ok_or_else(|| anyhow::anyhow!("Not connected to server"))?;
        write_frame(stream, &msg).await?;
        let received_msg = read_frame(stream).await?;
        println!("received msg {:?}", received_msg);
        Ok(())
    }
}