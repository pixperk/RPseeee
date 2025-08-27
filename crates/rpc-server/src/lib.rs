use rpc_core::framing::{read_frame, write_frame};
use tokio::net::{TcpListener, TcpStream};

pub mod registry;
pub mod handler;

pub struct Server {
    listen_addr: String,
}

pub fn new_server(listen_addr: String) -> Server {
    Server { listen_addr }
}

impl Server {
    pub async fn start(&self) -> anyhow::Result<()> {
        let lis = TcpListener::bind(&self.listen_addr).await?;
        println!("server running on {}", self.listen_addr);
        Server::listen_and_accept(self, lis).await

    }

    async fn listen_and_accept(&self, lis: TcpListener) -> anyhow::Result<()> {
        loop {
            let (mut socket, addr) = lis.accept().await?;
            println!("new connection from {:?}", addr);

            tokio::spawn(async move {
                if let Err(e) = Server::handle_client(&mut socket).await {
                    println!("error handling client: {:?}", e);
                }
            });
        }
    }

    async fn handle_client(stream: &mut TcpStream) -> anyhow::Result<()> {
        loop {
            let msg = read_frame(stream).await?;
            println!("got msg {:?}", msg);
            println!("payload : {}", msg.get_payload_as_str()?);
            write_frame(stream, &msg).await?;
        }
    }
}
