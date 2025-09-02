mod another;
use std::fmt::Debug;

fn main() {
    let human = Human { name: "Human1".to_string(), age: 45, gender: Gender::Male, };
    println!("Hello from Human: {}, age: {}, gender: {:?}. Who lives inside main.rs", human.name, human.age, human.gender);
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
