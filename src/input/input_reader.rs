use std::io;

pub fn get_parsed_input() -> u16 {
    parse_u16(get_input())
}

pub fn get_input() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Could not read line.");

    input
}

pub fn parse_u16(input: String) -> u16 {
    let trim_input = input.trim();
    trim_input.parse::<u16>()
        .expect("Failed to parse input.")
}