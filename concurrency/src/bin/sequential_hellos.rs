use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
  count: usize,
}

fn main() {
  let args = Args::parse();

  for i in 0..args.count {
    println!("Bonjour n.{i}");
    println!("Au revoir n.{i}");
  }
}
