#![allow(unused)]

enum Animal {
    Cat,
    Dog,
    Duck,
    Mouse,
}

fn main() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("tree"),
        _ => println!("other"),
    }

    match x {
        1 | 2 | 3 => println!("1 or 2 or 3"),
        _ => println!("other"),
    }

    match x {
        i @ 1..=10 => println!("matched {i}"),
        _ => println!("other"),
    }

    let animal = Animal::Cat;

    let animal_sound = match animal {
        Animal::Cat => "moew",
        Animal::Dog => "woof",
        Animal::Duck => "quak",
        _ => "?",
    };

    println!("animal sound {animal_sound}");

    let x: Option<i32> = Some(1);

    match x {
        Some(v) => println!("Some {v}"),
        None => println!("none"),
    }

    let x: Result<i32, String> = Ok(10);

    match x {
        Ok(v) => println!("Ok {v}"),
        Err(msg) => println!("Err {msg}"),
    }
}
