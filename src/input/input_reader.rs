use std::io;

use crate::data::coin::Coin;

pub fn get_parsed_input() -> u32 {
    parse_u32(get_input())
}

pub fn get_input() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Could not read line.");

    input
}

pub fn parse_u32(input: String) -> u32 {
    let trim_input = input.trim();
    trim_input.parse::<u32>()
        .expect("Failed to parse input.")
}

pub fn parse_coin(input: String) -> Coin {
    let trim_input = input.trim();
    match trim_input.as_ref() {
      "gp" | "gold" => Coin::Gold,
      "sp" | "silver" => Coin::Silver,
      "cp" | "copper" => Coin::Copper,
      _ => panic!("Invalid coinage")
    }
}

pub fn get_coin(operation: &str, input: String) -> Coin {
        println! ("{} what? (gp, sp, cp): ", operation);
        parse_coin(input)
}

  