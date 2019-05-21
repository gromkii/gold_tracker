use std::io;

pub fn get_input() -> String {
  let mut guess = String::new();
  
  io::stdin().read_line(&mut guess)
    .expect("Failed to read line");

  guess
}