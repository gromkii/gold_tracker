//! main.rs

mod input;
mod data;

use input::input_reader;
use input::command;
use input::command::Command;
use data::Currency;

fn main() {
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



