#![allow(unused)]

trait Animal {
    fn speak(&self) -> String;
}

struct Cat {}
struct Dog {}

impl Animal for Cat {
    fn speak(&self) -> String {
        "meow".to_string()
    }
}

impl Animal for Dog {
    fn speak(&self) -> String {
        "woof".to_string()
    }
}

fn greet(animal: &impl Animal) {
    println!("static: {}", animal.speak());
}

fn greet_dyn(animal: &dyn Animal) {
    println!("dynamic: {}", animal.speak());
}

fn create_animal() -> impl Animal {
    Dog {}
}

fn create_animal_dyn(number: u32) -> Box<dyn Animal> {
    match number {
        0..=10 => Box::new(Dog {}),
        _ => Box::new(Cat {}),
    }
}

fn main() {
    let cat = Cat {};
    let dog = Dog {};

    greet(&cat);
    greet(&dog);
    greet_dyn(&dog);

    let animal = create_animal();

    println!("animal speak: {}", animal.speak());

    let animal_type = "dog";

    let animal: &dyn Animal = match animal_type {
        "dog" => &Dog {},
        _ => &Cat {},
    };

    greet_dyn(animal);

    let animal = create_animal_dyn(100);
    println!("create_animal_dyn: {}", animal.speak());
}
