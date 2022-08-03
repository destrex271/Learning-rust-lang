use std::io;

fn main(){
    let mut buffer = String::new();
    println!("Enter the number of iterations you want: ");
    io::stdin().read_line(&mut buffer).expect("Failed to access Stdin");

    let limit: isize = buffer.trim().parse().expect("Input is not an integer");

    let mut i: isize = 1;
    loop{
        println!("Loop Counter: {}", i);
        if i == limit{
            break;
        }
        i += 1;
    }

    
}