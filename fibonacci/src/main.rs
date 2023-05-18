use std::io;

fn main() {
    println!("Welcome to Fibonacci Generator!");
    println!("What nth number of Fibonacci do you want?");

    //The nth number to generate
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Error reading from stdin");
    
    //Converting to u32
    let number: u32 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error parsing.");
            0
        }
    };
    
    let result = generate_fib(number);
    println!("{result}");
}

fn generate_fib(number: u32) -> u32 {
    if number == 0 || number == 1 {
        return 1;
    } else {
        return generate_fib(number - 1) + generate_fib(number - 2);
    }
}
