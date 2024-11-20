use std::time::Duration;

pub async fn is_open (host: &str, port: u16, timeout: u64) -> bool {
  matches! (
    tokio::time::timeout(
      Duration::from_secs(timeout),
      tokio::net::TcpStream::connect(
        format!("{host}:{port}")
      )
    ).await,
    Ok(Ok(_))
  )
}
