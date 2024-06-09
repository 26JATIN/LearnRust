use std::io;

fn main() {
    println!("Enter a number: ");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let a: f64 = input.trim().parse().expect("Please type a valid number!");

    println!("The square root of {} is {}", a, a.sqrt());
}
