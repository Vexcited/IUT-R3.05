fn main () {
  // 2.1
  let x = 10.0;
  let y = 20.0;
  let average_value1 = average(x, y);
  let average_value2 = average(x, y);
  println!("average1: {average_value1}");
  println!("average2: {average_value2}");

  // 2.2
  let my_rectangle = Rectangle {
    length: 4.5,
    width: 2.7
  };

  let rectangle_perimeter1 = perimeter(&my_rectangle);
  let rectangle_perimeter2 = perimeter(&my_rectangle);
  println!("perimeter1: {rectangle_perimeter1}");
  println!("perimeter2: {rectangle_perimeter2}");

  // 3.1
  print_references();

  // 3.2
  let mut x = 10.0;
  let mut y = 20.0;
  swap(&mut x, &mut y);
  println!("x: {x}, y: {y}");
}

fn average (a: f64, b: f64) -> f64 {
  (a + b) / 2.0
}

struct Rectangle {
  length: f64,
  width: f64
}

// ici on utilise le passage par référence
fn perimeter (rectangle: &Rectangle) -> f64 {
  2.0 * (rectangle.length + rectangle.width)
}

fn print_references () {
  let x = 18;

  // 3.1 / 1. et 3.
  let ref1 = &x;
  let ref2 = &x;
  // ne fonctionne pas :
  // let ref3 = &mut x;
  //            ^ cannot borrow `x`
  println!("ref1: {ref1}, ref2: {ref2}");
  
  // 3.1 / 4.
  // ne fonctionne pas :
  // let ref3 = &mut x;
  // let ref4 = &mut x;
  //            ^ cannot mutate immutable variable
  // println!("ref3: {ref3}, ref4: {ref4}");
}

fn swap (x: &mut f64, y: &mut f64) {
  let temp = *x;
  *x = *y;
  *y = temp;
}
