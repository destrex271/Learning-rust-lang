fn main() {
    let x = 5;
    // x = 6; Wont Work as rust variables are immutable
    let mut y = 6;
    y = 10;

    const CONST_VAR : u32 = 10;

    let z = 10;
    println!("{} is the original value before shadowing", z);

    let z = z + 1;
    println!("{} is the shadowed value", z);

    println!("{} immuatble var\n{} mutable var\n{} 'const' variable", x, y, CONST_VAR);
}
