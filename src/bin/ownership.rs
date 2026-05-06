#![allow(unused)]

fn take(s: String) {}

fn copy(i: i32) {}

fn main() {
    // Owner of "rust" is s
    let s = String::from("rust");
    // The ownership moved to s1
    let s1 = s;

    // The print can not be used, since the ownership moved
    // println!("{s}");
    println!("{s1}");

    // Owner of -1 is i
    let i = -1;

    // Owner of -1 is i
    let i1 = i;

    // The print can be used, since -1 was cloned
    println!("{i}, {i1}");

    // Owner of "rust" is s
    let s = String::from("rust");

    if (true) {
        // New scope
        // The ownership moved to another scope
        s;
    } // s is dropped

    // The print can not be used, since s was dropped
    // println!("{s}");

    // Owner of "rust" is s
    let s = String::from("rust");

    {
        let s1 = s;
    } // s1 is dropped

    // Owner of "rust" is s
    let s = String::from("rust");

    // The ownership moved to take()
    take(s);

    // The print can not be used, since s was moved
    // println!("{s}");

    // Owner of -1 is i
    let i = -1;

    copy(i);

    // The print can be used, since -1 was cloned 
    println!("{i}");
}
