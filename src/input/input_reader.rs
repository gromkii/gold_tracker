use std::io;

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