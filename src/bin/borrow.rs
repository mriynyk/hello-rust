#![allow(unused)]

fn main() {
    let s = String::from("rust");
    let s1 = &s;
    let s2 = &s;

    // Immutable borrow
    let s = String::from("rust");
    let s1 = &s;
    // any number of read-only access to a value
    let s2 = &s;
    let s3 = s2;
    let s4 = s2;

    // Mutable borrow
    let mut s = String::from("rust");
    // only one mutable reference at a time
    let s1 = &mut s;
    // let s2 = &mut s; // can not create here
    s1.push_str("🦀");
    // let s2 = &mut s; // can create here
    println!("{s}");

    // Can not borrow immutable and mutable simultaneously
    let mut s = String::from("rust");
    let s1 = &s;
    // let s2 = &mut s; // can not create here
    println!("{s1}");
    let s2 = &mut s; // can create here
    // let s3 = &s; // can not create here
    println!("{s2}");
    let s3 = &s; // can create here

    // Reference must not outlive the value
    let s = String::from("rust");
    let s1 = &s;
    // std::mem::drop(s); // can not be doped here
    println!("{s1}");
    std::mem::drop(s); // can be doped here
}

// fn f(s: String) -> &String {
//     &s
// }
