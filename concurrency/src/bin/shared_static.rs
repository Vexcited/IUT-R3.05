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
  let hello: &str = "Bonjour";

  for i in 0..args.count {
    let task = tokio::spawn(async move {
      println!("{hello} n.{i}");
      println!("Au revoir n.{i}");
    });

    tasks.push(task);
  }

  for task in tasks {
    task.await.unwrap();
  }
}

// Concernant la variable `hello`, dans quel segment de mémoire est-elle stockée ?
// > Dans le segment de données statiques.
// Quelle est sa durée de vie ?
// > La durée de vie de la variable `hello` est celle du programme.