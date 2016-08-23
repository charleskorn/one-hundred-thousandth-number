extern crate one_hundred_thousandth_number;

use one_hundred_thousandth_number::generator::generate_nth_number;
use std::env::args;

fn main() {
    let raw_n = args().nth(1).unwrap_or("100000".to_string());

    if let Ok(n) = raw_n.parse::<u32>() {
        match generate_nth_number(n) {
            Ok(value) => println!("{}", value),
            Err(message) => println!("Error: {}", message),
        }
    } else {
        println!("Error: please provide a positive integer")
    }
}
