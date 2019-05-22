//! command.rs
//! Handles user input, and dispatches events.

mod input_reader;

enum Command {
  add,
  subtract,
  exit
}

pub fn get_command(input: String) -> Command {
  let str = input_reader::get_input().trim();

  match str {
    "add" => Command::add,
    "subtract" => Command::subtract,
    "exit" => Command::exit,
  }
}