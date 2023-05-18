fn main() {
    let s = String::from("hello");
    take_ownership(s.clone());

    //Might be invalid?
    println!("{s}");

}

fn take_ownership(some_string: String) {
    println!("{some_string}");
} //drop is called
