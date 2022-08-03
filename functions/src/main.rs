fn main() {
    let _x = String::from("Hello Akshat");
    let y = String::from("");
    println!("{}", last_cagr(y));
}

fn last_cagr(string: String) -> char {
    if string.is_empty(){
        return '🚨';
    }
    string.as_bytes()[string.len() - 1] as char
    //string.chars().next_back().unwrap()
}
