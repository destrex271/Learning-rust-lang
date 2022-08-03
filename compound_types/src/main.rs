fn main() {
    let array : [u32; 3] = [1, 2, 3];

    let first_element = array[0];
    // let overbound = array[1000];

    println!("{} is length array.\n{} first element",array.len(),first_element);

    let tup : (u32, &str, bool) = (12, "Akshat", false);
    let first_tup = tup.2;

    println!("{}", first_tup);

}
