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
  let socket = UdpSocket::bind(format!("{}:{}", args.host, args.port)).await?;
  
  let mut buf = vec![0u8; 1024];
  let mut to_send = None;
  
  loop {
    // if there's a message then we try to send it back to the original source, waiting
    // until it's writable and we're able to do so.
    if let Some((size, peer)) = to_send {
      let amt = socket.send_to(&buf[..size], &peer).await?;
      println!("echo {amt}/{size} bytes to {peer}");
    }

    // if we're here then `to_send` is `None`,
    // so we take a look for the next message we're going to echo back.
    to_send = Some(socket.recv_from(&mut buf).await?);
}
}
