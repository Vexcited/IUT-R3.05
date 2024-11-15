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

  for i in 0..args.count {
    let task = tokio::spawn(async move {
      println!("Bonjour n.{i}");
      println!("Au revoir n.{i}");
    });

    tasks.push(task);
  }

  for task in tasks {
    task.await.unwrap();
  }
}

// 2.1 : Si on oublie d'attendre, le programme se termine avant
// que les tâches aient le temps de s'exécuter.

// 2.2 : Il est normal que l'ordre soit aléatoire car
// les tâches sont exécutées en parallèle.
