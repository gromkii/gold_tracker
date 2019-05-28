//! command.rs
//! Handles user input, and dispatches events.

use crate::data::coin::Coin;
use crate::data::coinage::Coinage;
use crate::data::coin_held::CoinHeld;

use crate::input_reader;
use crate::input_reader::get_coin;

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

pub fn handle_command(command: &Command, coin_held: &mut CoinHeld) {
  match command {
    Command::Init => handle_init(),
    Command::Add => handle_math(command, coin_held),
    Command::Subtract => handle_math(command, coin_held),
    Command::List => handle_list(&coin_held),
    Command::Exit => handle_exit(),
    Command::Error => handle_error(),
  }
}

fn handle_init() {
  println!("Initializing CLI");
}

fn handle_math(command: &Command, coin_held: &mut CoinHeld) {
  println!("Coin type: ");
  let coin = get_coin(command.to_string(), input_reader::get_input());

  let coinage: &Coinage = match coin {
    Coin::Gold => &coin_held.gold,
    Coin::Silver => &coin_held.silver,
    Coin::Copper => &coin_held.copper,
  };

  println!("Amount to {}", command.to_string());
  let amount = input_reader::parse_u32(input_reader::get_input());

  if command == &Command::Add {
    coinage.add_amount(amount);
  } else if command == &Command::Subtract {
    coinage.sub_amount(amount);
  }
}

fn handle_exit() {
  println!("Exiting");
}

fn handle_error() {
  println!("Invalid command");
}

fn handle_list(coin_held: &CoinHeld) {
  println!("{}", coin_held.list_coinage());
} 