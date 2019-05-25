//! main.rs

mod input;
use input::input_reader;
use input::command;
use input::command::Command;

mod data;
use data::coinage::Coinage;
use data::coin::Coin;

mod math;

fn main() {
    input_loop();
}

fn input_loop() {
    let mut user_command: Command = Command::Init;
    let coin: Vec<u32> = init_gold();
    let mut c: Coinage = Coinage::new(coin[0], coin[1], coin[2]);
    
    while user_command != Command::Exit {
        println!("Enter command (add, subtract, exit)");
        user_command = command::get_command(input_reader::get_input());
        command::handle_command(&user_command, &mut c);
    }
}

fn init_gold() -> Vec<u32> {
    println!("Enter amount of Gold: ");
    let gold: u32 = input_reader::get_parsed_input();
    println!("Enter amount of Silver: ");
    let silver: u32 = input_reader::get_parsed_input();
    println!("Enter amount of Copper: ");
    let copper: u32 = input_reader::get_parsed_input();

    vec!(gold, silver, copper)
}



