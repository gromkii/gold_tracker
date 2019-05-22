//! main.rs

mod input;
mod data;

use input::input_reader;
use input::command;
use input::command::Command;
use data::Currency;

fn main() {
    init();
}

fn init() {
    println!("Enter total gold: ");
    let total_gold: u16 = input_reader::get_parsed_input();
    println!("Enter total silver: ");
    let total_silver: u16 = input_reader::get_parsed_input();
    println!("Enter total copper: ");
    let total_copper: u16 = input_reader::get_parsed_input();

    println!("Total gold held: {}", total_gold);
    println!("Total silver held: {}", total_silver);
    println!("Total copper held: {}", total_copper);

    let c: Currency = Currency::new(total_gold, total_silver, total_copper);

    println!("Currency: {:?}", c.get_currency());

    input_loop();
    
}

fn input_loop() {
    let mut user_command: Command = Command::Init;
    while user_command != Command::Exit {
        println!("Enter command (add, subtract, exit)");
        user_command = command::get_command(input_reader::get_input());
        command::handle_command(&user_command);
    }
}



