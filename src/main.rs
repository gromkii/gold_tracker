//! main.rs

mod input;

use input::input_reader;

fn main() {
    println!("Enter total gold: ");
    let total_gold: u16 = input_reader::get_parsed_input();

    println!("Total gold held: {}", total_gold);
}



