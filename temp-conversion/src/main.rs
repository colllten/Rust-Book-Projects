use std::io;
fn main() {
    println!("Welcome to F -> C");
    let mut fahrenheit = String::new();
    loop {
        println!("Please enter a temperature in fahrenheit.");
        io::stdin()
            .read_line(&mut fahrenheit)
            .expect("Problem reading from stdin");        
        let fahrenheit: f64 = match fahrenheit.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error parsing number");
                continue;
            }
        };

        println!("Converting to Celsius...");
        let celsius = (fahrenheit - 32.0) * (5.0 / 9.0);
        println!("{fahrenheit}F is {celsius}C");
        break;
    }
}
