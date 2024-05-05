use std::io;

mod multiplier;

fn main() {
    println!("Enter a number:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let number: i32 = input.trim().parse().expect("Invalid input");

    let result = multiplier::multiply_by_five(number);
    println!("{} multiplied by 5 is: {}", number, result);
}