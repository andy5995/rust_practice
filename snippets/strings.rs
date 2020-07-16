use std::io;

fn main() {

  let mut name = String::new();
  println!("Enter a name...");
  io::stdin().read_line(&mut name).expect("Failed to read line");
  // let guess: u32 = guess.trim().parse().expect("Please type a number!");
  let name: String = name.trim().to_string();
  let name2 = print_name(name);
  println!("Is name {} out of scope?", name2);

  let s2 = name2;
  println!("{}, world!", s2);
}

fn print_name(name:String) ->String {
  println!("Name is {}", name);
  name
}
