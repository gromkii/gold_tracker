//! command.rs
//! Handles user input, and dispatches events.

#[derive(PartialEq)]
pub enum Command {
  Add,
  Subtract,
  Exit,
  Error,
  Init,
}

pub fn get_command(input: String) -> Command {
  let trim_input = input.trim();
  match trim_input.as_ref() {
    "add" => Command::Add,
    "subtract" => Command::Subtract,
    "exit" => Command::Exit,
    _ => Command::Error
  }
}

pub fn handle_command(command: &Command) {
  match command {
    Command::Init => handle_init(),
    Command::Add => handle_add(),
    Command::Subtract => handle_sub(),
    Command::Exit => handle_exit(),
    Command::Error => handle_error()
  }
}

fn handle_init() {
  println!("Initializing Prompt");
}

fn handle_add() {
  println!("Adding");
}

fn handle_sub() {
  println!("Subtracting");
}

fn handle_exit() {
  println!("Exiting");
}

fn handle_error() {
  println!("Invalid Command");
}