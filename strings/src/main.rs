
fn main(){
    let buffer = String::from("I Am Akshat!\nHelloooo!");
    // io::stdin().read_line(&mut buffer).unwrap();
    println!("First Line is : {}", first_line(&buffer));
}

pub fn first_line(string: &String) -> String{
    String::from(string.lines().next().unwrap())
}