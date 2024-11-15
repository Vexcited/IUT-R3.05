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
  let mut total = 0;

  for _ in 0..args.count {
    let task = tokio::spawn(async move {
      total += 1;

      println!("Bonjour n.{total}");
      println!("Au revoir n.{total}");
    });

    tasks.push(task);
  }

  for task in tasks {
    task.await.unwrap();
  }
}

// 4. Qu'observez vous ?
// > La variable `total` n'est pas incrémentée correctement.
