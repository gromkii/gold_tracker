//! main.rs

mod input;
use input::input_reader;
use input::input_reader::get_parsed_input as parsed_input;
use input::command;
use input::command::Command;

mod data;
use data::coinage::Coinage;
use data::coin_held::CoinHeld;
use data::coin::Coin;

fn main() {
    input_loop();
}

fn input_loop() {
    let mut user_command: Command = Command::Init;
    let mut c: CoinHeld = init_gold();
    
    while user_command != Command::Exit {
        println!("Enter command (add, subtract, exit)");
        user_command = command::get_command(input_reader::get_input());
        command::handle_command(&user_command, &mut c);
    }
}

fn init_gold() -> CoinHeld {
    println!("Enter amount of Gold: ");
    let gold = Coinage::new(Coin::Gold, parsed_input());
    
    println!("Enter amount of Silver: ");
    let silver: Coinage = Coinage::new(Coin::Silver, parsed_input());
    
    println!("Enter amount of Copper: ");
    let copper: Coinage = Coinage::new(Coin::Copper, parsed_input());

    CoinHeld::new(gold, silver, copper)
}



