//! command.rs
//! Handles user input, and dispatches events.

use crate::data::coinage::Coinage;
use crate::data::coin::Coin;

use crate::input_reader;

use crate::math::set_coin;

#[derive(PartialEq)]
pub enum Command {
  Add,
  Subtract,
  Exit,
  Error,
  Init,
  List,
}

impl Command {
  pub fn to_string(&self) -> String {
    match self {
      &Command::Add => String::from("Add"),
      &Command::Subtract => String::from("Subtract"),
      &Command::Exit => String::from("Exit"),
      &Command::Init => String::from("Init"),
      &Command::List => String::from("List"),
      &Command::Error => String::from("Error")
    }
  }
}

pub fn get_command(input: String) -> Command {
  let trim_input = input.trim();
  match trim_input.as_ref() {
    "add" => Command::Add,
    "subtract" => Command::Subtract,
    "exit" => Command::Exit,
    "list" => Command::List,
    _ => Command::Error
  }
}

pub fn handle_command(command: &Command, coinage: &mut Coinage) {
  match command {
    Command::Init => handle_init(),
    Command::Add => handle_math(coinage, command),
    Command::Subtract => handle_math(coinage, command),
    Command::Exit => handle_exit(),
    Command::Error => handle_error(),
    Command::List => handle_list(coinage),
  }
}

fn handle_init() {
  println!("Initializing Prompt");
}

fn handle_math(c: &mut Coinage, command: &Command) {
  println!("{} what?", command.to_string());
  let coin: &Coin = &input_reader::get_coin("Add", input_reader::get_input());
  
  println!("Amount");
  let amount = input_reader::parse_u32(input_reader::get_input());
  set_coin(c, coin, amount, &command);
  
  println!("{}", c.list_coinage());
}

fn handle_exit() {
  println!("Exiting");
}

fn handle_error() {
  println!("Invalid Command");
}

fn handle_list(coinage: &Coinage) {
  println!("{}", coinage.list_coinage());
}