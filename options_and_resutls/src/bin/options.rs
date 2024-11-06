fn main () {
  let sentence1 = "Bonjour Limoges";
  let sentence2 = "";

  println!("2.1 /");
  print_first_word(sentence1);
  print_first_word(sentence2);
  
  println!("2.2 /");
  print_first_word2(sentence1);
  // commenté sinon panic car chaîne vide
  // print_first_word2(sentence2);

  println!("2.3 /");
  iterate_over_words(sentence1);
  iterate_over_words(sentence2);
}

fn print_first_word (str: &str) {
  match str.split_whitespace().next() {
    Some(word) => {
      println!("Premier mot : {word}")
    },
    None => {
      println!("Chaîne vide")
    }
  }
}

fn print_first_word2 (str: &str) {
  let word = str.split_whitespace().next().expect("La chaîne doit être non vide");
  println!("Premier mot: {word}");
}

fn iterate_over_words (str: &str) {
  for word in str.split_whitespace() {
    println!("Mot : {word}");
  }
}