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
    Command::Add => handle_add(coinage),
    Command::Subtract => handle_sub(),
    Command::Exit => handle_exit(),
    Command::Error => handle_error(),
    Command::List => handle_list(&coinage),
  }
}

fn handle_init() {
  println!("Initializing Prompt");
}

fn handle_add(c: &mut Coinage) {
  println! ("Add what? (gp, sp, cp): ");
  let coin: &Coin = &input_reader::parse_coin(input_reader::get_input());

  println!("How much?");
  let amount = input_reader::parse_u32(input_reader::get_input());
  set_coin(c, coin, amount);
  
  println!("{}", c.list_coinage());
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

fn handle_list(coinage: &Coinage) {
  println!("{}", coinage.list_coinage());
}