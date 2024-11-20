use clap::Parser;
use port_scanner::is_open;
use std::{sync::Arc, time::Instant};
use tokio::{task::JoinHandle, sync::RwLock};

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
  let open_ports = Arc::new(RwLock::new(Vec::<u16>::new()));
  let mut tasks: Vec<JoinHandle<()>> = Vec::new();

  for port in args.min..=args.max {
    let host = args.host.clone();
    let timeout = args.timeout;
    let open_ports = Arc::clone(&open_ports);

    let task = tokio::spawn(async move {
      if is_open(&host, port, timeout).await {
        let mut open_ports = open_ports.write().await;
        open_ports.push(port);
      }
    });

    tasks.push(task);
  }

  for task in tasks {
    task.await.unwrap();
  }

  let duration = start.elapsed();
  let open_ports = open_ports.read().await;
  
  for port in open_ports.iter() {
    println!("port {} is open", port);
  }

  println!("Time elapsed: {:?}", duration);

  Ok(())
}

// 4. Comparez le temps avec le balayage séquentiel.
// > Le temps qu'on obtient maintenant est d'environ 1.002995667 secondes

// 4. Balayez tous les ports de 1 à 65535 avec un timeout de 30 secondes.
// > port 22 is open
// > port 80 is open
// > port 2000 is open
// > port 5060 is open
// > port 9929 is open
// > port 31337 is open
// Le temps qu'on obtient est d'environ 30.051525292s secondes.