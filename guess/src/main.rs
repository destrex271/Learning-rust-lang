use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    let val = String::new(rand::thread_rng().gen_range(1..=100));
    println!("\t\tGuess Game!");
    println!("Please enter your guess:");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("ERROR : Cannot process input!");
    println!("{} is the guess", guess);

    match guess.cmp(&val) {
        Ordering::Less => println!("Smallll"),
        Ordering::Equal => println!("Equal"),
        Ordering::Greater => println!("Larger"),
    }

}
