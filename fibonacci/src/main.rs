use std::io;

fn main() {
    println!("Welcome to Fibonacci Generator!");
    println!("What nth number of Fibonacci do you want?");

    //The nth number to generate
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Error reading from stdin");
    println!("{number}");
}
