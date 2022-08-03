fn main() {
    let mut x : i32 = 5;
    println!("The value of x is : {}", x);
    x = 32;
    println!("New value of x is : {}", x);

    let y : bool = true;
    println!("The value of y is : {}", y);
    let y : bool = !y;
    println!("New Value of y is : {}", y);

    const STRING: &str = "Hello Akshat";
    println!("{}!",STRING);

}
