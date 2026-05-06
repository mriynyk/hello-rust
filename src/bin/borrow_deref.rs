#![allow(unused)]

fn modify(s: &mut String) {
    // deref doesn't take ownership
    *s += "?";
}

fn main() {
    let mut s = String::from("rust");
    let s1 = &mut s;
    *s1 += "?";

    println!("{:?}", s.len());

    let mut s = String::from("rust");
    println!("{:?}", s.len());

    let s1 = &mut s;
    *s1 += "?";

    println!("{:?}", s.len());
    // modify(&mut s);

    // deref automatically
    let x = 1;
    let y = &x;
    let z = &x;
    let w = y + z; // let w = *y + *z
}
