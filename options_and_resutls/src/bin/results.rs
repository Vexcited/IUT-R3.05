fn main () -> anyhow::Result<()> {
  let string1 = "-17";
  let string2 = "Tux";

  // 3.1
  println!("3.1 /");
  convert_to_int1(string1);
  convert_to_int1(string2);
  
  // 3.2
  println!("3.2 /");
  convert_to_int2(string1);
  // commenté sinon panic car chaîne non entier
  // convert_to_int2(string2);
  
  // 3.3
  println!("3.3 /");
  convert_to_int3(string1)?;
  // va panic la fonction main
  // il faut la décommenter pour ne pas avoir le panic
  convert_to_int3(string2)?;

  Ok(())
}

fn convert_to_int1 (str: &str) {
  match str.parse::<i32>() {
    Ok(value) => {
      println!("Le carré de {value} vaut {}", value * value);
    },
    Err(_) => {
      println!("{str} n'est pas un nombre entier.");
    }
  }
}

fn convert_to_int2 (str: &str) {
  let value: i32 = str.parse().expect("pas un nombre entier");
  println!("Le carré de {value} vaut {}", value * value);
}

fn convert_to_int3 (str: &str) -> anyhow::Result<()> {
  let value: i32 = str.parse()?;
  println!("Le carré de {value} vaut {}", value * value);
  Ok(())
}