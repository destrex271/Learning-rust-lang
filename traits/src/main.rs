pub struct Person{
    name: String
}
pub struct Cat{
    name: String
}
pub trait Eat{
    fn eat(&self);
}
impl Eat for Person{
    fn eat(&self){
        println!("Person {} Eating..", self.name);
    }
}
impl Eat for Cat{
    fn eat(&self){
        println!("Cat {} Eating...", self.name);
    }
}

fn main(){
    let person = Person{
        name: "Person 1".to_string()
    };
    let cat = Cat{
        name: "Cat 1".to_string()
    };
    person.eat();
    cat.eat();
}