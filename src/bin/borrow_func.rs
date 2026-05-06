#![allow(unused)]

// Borrow and function

fn take(s: String) {
    println!("take {s}");
}

fn borrow(s: &str) {
    println!("borrow {s}");
}

fn borrow_mut(s: &mut String) {
    s.push_str("🦀");
}

fn print_len(s: String) {
    println!("length = {}", s.len());
}

fn print_len_return_ownership(s: String) -> String {
    println!("length = {}", s.len());

    s
}

fn print_len_borrow_ownership(s: &str) {
    println!("length = {}", s.len());
}

fn main() {
    // Take ownership
    let s = String::from("rust");
    take(s);

    let s = String::from("rust");
    borrow(&s);

    let mut s = String::from("rust");
    borrow_mut(&mut s);
    println!("{s}");

    // Modify a function in 3 steps
    // 1. Take ownership
    let s = String::from("rust");
    print_len(s);
    // 2. Return ownership
    let s = String::from("rust");
    let s = print_len_return_ownership(s);
    println!("{s}");
    // 3. Borrow
    let s = String::from("rust");
    print_len_borrow_ownership(&s);
    println!("{s}");
}
