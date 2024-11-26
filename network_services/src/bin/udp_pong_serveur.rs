use clap::Parser;
use tokio::net::UdpSocket;

#[derive(Parser, Debug)]
struct Args {
  #[clap(long)]
  host: String,

  #[clap(long)]
  port: u16, 
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let args = Args::parse();
  let mut buf = vec![0u8; 32];
  let socket = UdpSocket::bind(format!("{}:{}", args.host, args.port)).await?;
  
  loop {
    let (_, addr) = socket.recv_from(&mut buf).await?;

    socket.send_to(b"PONG\n", addr).await?;
  }
}
