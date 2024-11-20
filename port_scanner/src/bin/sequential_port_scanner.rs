use clap::Parser;
use port_scanner::is_open;
use std::time::Instant;

#[derive(Parser, Debug)]
struct Args {
  /// Minimum port number to scan
  #[clap(long)]
  min: u16,

  /// Maximum port number to scan
  #[clap(long)]
  max: u16,

  /// Host to scan
  #[clap(long)]
  host: String,

  /// Timeout for each port scan in seconds
  #[clap(short, long, default_value = "1")]
  timeout: u64,
}


#[tokio::main]
async fn main () -> anyhow::Result<()> {
  let args = Args::parse();
  let start = Instant::now();

  for port in args.min..=args.max {
    if is_open(&args.host, port, args.timeout).await {
      println!("port {} is open", port);
    }
    else {
      println!("port {} is closed", port);
    }
  }

  let duration = start.elapsed();
  println!("Time elapsed: {:?}", duration);

  Ok(())
}

// 4. Notez le temps que le prend le balayage séquentiel.
// > Le temps est d'environ 34 secondes.
