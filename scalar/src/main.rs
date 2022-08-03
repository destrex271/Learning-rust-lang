fn main() {
    let a : u8 = 5;
    let b : u8 = 1;

    let x : f64 = 2.0;
    let y : f64 = 3.0;

    let sum : u8 = a + b;
    let sum2 : f64 = (a + b).into();
    // let sum3 : f64 = x + y.into();
    let dif1 : u8 = a - b;
    let dif2 : f64 = x - y;
    let prod = 4 * a;
    let quotient = 9.0/y;
    let remainder = a % b;

    let _t : bool = true;
    let _f : bool = false;

    let c: char = '😂';

    println!("Sum 1 : {}\nSum 2 : {}\nDif 1 : {}\nDif 2 : {}\nProd : {}\nQuotient : {}\nRemainder : {}\nChar : {}", sum, sum2, dif1, dif2, prod, quotient, remainder, c);

}
