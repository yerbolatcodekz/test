mod another;
use std::fmt::Debug;

fn main() {
    let human = Human { name: "Human1".to_string(), age: 45, gender: Gender::Male, };
    let animal = Animal { name: "Cow".to_string(), };
    println!("Hello from Human: {}, age: {}, gender: {:?}. Who lives inside \"main.rs\" file.", human.name, human.age, human.gender);

    println!("I have a animal. It's name is {}.", animal.name);

    println!("Human has an cow, with name {}.", animal.name);
    another::new();
} 
#[derive(Debug)]
enum Gender {
    Male,
    Famale,
}
struct Human {
    name: String,
    age: i32,
    gender: Gender 
}

struct Animal {
    name: String,
}
