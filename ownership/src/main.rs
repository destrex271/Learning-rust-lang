fn main() {
    let x = String::from("Hello World!");
    println!("{x}");
    print(x);
    // println!("{x}"); // program panics for types like string as they do not implement the copy trait
}

fn print(string: String)->(){
    println!("{string}");
}
