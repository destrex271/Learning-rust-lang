use std::io;

fn main(){
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed To access stdin");
    let x: i32 = buffer.trim().parse().expect("Input is not an integer");

    let result : &str = if x % 2 == 0{
        "Even"
    }else{
        "Odd"
    };

    println!("The Result is {}", result);

    match x % 2{
        0 => println!("Divisible by 2"),
        _ => println!("Not divisible by 2")
    }
}