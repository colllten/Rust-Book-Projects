use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please enter your guess.");

        //Use 'let' to create a constant
        //Variables are immutable by default
        let mut guess = String::new(); //Add 'mut' to change its value

        //'::' means it is an associated function with that type

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Number not entered!");
                continue;
            } 
        };

        //References are immutable by default, so you must add '&mut' before the variable name

        println!("You guessed: {}", guess);
        //You can also do '{}' and specify what goes in there after String like printf()

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
