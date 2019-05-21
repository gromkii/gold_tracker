// main.rs
mod input_reader;

fn main() {
    println!("Gold Tracker");
    println!("Please enter your ammount of gold:");
    let gold: String = input_reader::get_input();

    println!("You are currently holding {}gp.", gold);
}
