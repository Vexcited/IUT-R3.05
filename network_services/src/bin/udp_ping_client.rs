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
  
  let socket = UdpSocket::bind("0.0.0.0:0").await?;
  socket.connect(format!("{}:{}", args.host, args.port)).await?;
  socket.send(b"PING\n").await?;

  let mut buf = vec![0u8; 32];
  let _ = socket.recv(&mut buf).await?;
  let response = std::str::from_utf8(&buf).unwrap();

  println!("{response}");
  Ok(())
}
