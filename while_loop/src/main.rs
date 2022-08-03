
use std::io;

fn countdown(upto: isize){
    let mut i = upto;
    while i >= 0{
        for _ in 1..i+1{
            print!(".");
        }
        println!();
        i -= 1;
    }
}

fn main(){
    let mut buffer = String::new();
    println!("Enter Countdown value");
    io::stdin().read_line(&mut buffer).expect("Failed to access stdin");
    let limit : isize = buffer.trim().parse().expect("Input is not an integer");
    countdown(limit);
    println!("TakeOff!");
}