
enum WebEvent{
    PageLoad,
    PageUnLoad,
    KeyPress(char),
    Paste(String),
    Click(x: i64, y: i64)
}

enum Option<T> {
    Some(T),
    None
}

fn main() {
    let quit = WebEvent::KeyPress(q);
    
    let somthing = Some(1);
}
