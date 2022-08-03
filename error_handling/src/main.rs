use std::fs::File;

fn main(){
    // panic!("Bye Bye!");
    // let v : Vec<i32> = vec![12,13,14,15];
    // println!("{}",v[6]);

    let fruits: Vec<&str> = vec!["Apple", "Banana", "Guava"];
    let first = fruits.get(0);
    println!("{:?} {:?}",first, fruits);

    let non_existing = fruits.get(99);
    println!("{:?}", non_existing);

    let f = File::open("hello.txt").unwrap();
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("Can't open file")
    // };
}
