use std::sync::Arc;

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
  let hello = Arc::new("Bonjour".to_string());

  for i in 0..args.count {
    let hello = Arc::clone(&hello);

    let task = tokio::spawn(async move {
      println!("{} n.{i}", hello);
      println!("Au revoir n.{i}");
    });

    tasks.push(task);
  }

  for task in tasks {
    task.await.unwrap();
  }
}
