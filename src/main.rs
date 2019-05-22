//! main.rs

mod input;
mod data;

use input::input_reader;
use data::Currency;

fn main() {
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
    println!("{}", c.get_gold());
}



