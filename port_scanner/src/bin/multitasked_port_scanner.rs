use clap::Parser;
use port_scanner::is_open;
use std::time::Instant;
use tokio::task::JoinHandle;

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
  let mut tasks: Vec<JoinHandle<()>> = Vec::new();

  for port in args.min..=args.max {
    let host = args.host.clone();
    let timeout = args.timeout;

    let task = tokio::spawn(async move {
      if is_open(&host, port, timeout).await {
        println!("port {} is open", port);
      }
    });

    tasks.push(task);
  }

  for task in tasks {
    task.await.unwrap();
  }

  let duration = start.elapsed();
  println!("Time elapsed: {:?}", duration);

  Ok(())
}

// 3. Comparez le temps avec le balayage séquentiel.
// > Le temps qu'on obtient maintenant est d'environ 1.003 secondes
// > contre d'environ 34 secondes pour le balayage séquentiel.
// > On a donc un gain de performance d'environ 33.997 secondes.

// 4. Balayez tous les ports de 1 à 65535 avec un timeout de 30 secondes.
// > port 22 is open
// > port 80 is open
// > port 2000 is open
// > port 5060 is open
// > port 9929 is open
// > port 31337 is open
// Le temps qu'on obtient est d'environ 30.051525292s secondes.