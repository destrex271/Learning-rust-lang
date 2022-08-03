fn main() {
    let mut x = String::from("Hello World!");
    // print(x.clone()); Creates copy, prevents ownership transfer but consumes memory
    print(&x); // Pass a 'reference' to the variable x on the stack as well as the heap and therefor ownership is not passed to the print function
    // We can also send a 'mutable' reference, if we want our function to make changes to the current value
    change(&mut x);
    // While borrowing a variable you can have either:
    // 1. A single mutable referrence
    // 2. A number of immutable references
    // And references must always be valid, like if a var is a ref to another var then if the
    // lifetime of that particular var expires then that of the reference also expires and therefore
    // the program panics
    print(&x);
}

fn print(string: &String)->(){
    println!("{string}");
}

fn change(string: &mut String){
    string.push_str("y");
}