use std::sync::Arc;
use tokio::sync::RwLock;

use clap::Parser;
use tokio::task::JoinHandle;

#[derive(Parser, Debug)]
struct Args {
  count: usize,
}

#[tokio::main]
async fn main() {
  let args = Args::parse();
  let mut tasks: Vec<JoinHandle<()>> = Vec::new();
  let count = Arc::new(RwLock::new(0));

  for _ in 0..args.count {
    let count = Arc::clone(&count);

    let task = tokio::spawn(async move {
      let mut count = count.write().await;
      
      println!("Bonjour n.{count}");
      println!("Au revoir n.{count}");

      *count += 1;
    });

    tasks.push(task);
  }

  for task in tasks {
    task.await.unwrap();
  }
}
