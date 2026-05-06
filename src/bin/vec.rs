#![allow(unused)]

fn main() {
    // new Vec<T>
    let v: Vec<i32> = vec![-1, 0, 1];
    let v: Vec<u32> = vec![1, 2, 3];
    let v: Vec<i32> = Vec::new();
    let v = vec![1u8, 2, 4];
    let v = vec![1u8, 1, 1, 1, 1, 1];
    let v = vec![1u8; 5];
    println!("v = {:?}, len = {}", v, v.len());

    // get
    let v = vec![1, 2, 3];
    let x = v[0];
    // but better
    let x = v.get(0);

    if let Some(if_val) = x {
        println!("if_val: {:?}", if_val);
    } else {
        println!("if_val: None");
    }

    let Some(val_else) = x else {
        println!("val_else: None");
        return ();
    };

    println!("val_else: {:?}", val_else);

    // update
    let mut v = vec![1, 2, 3];
    v[1] = 99;

    // push
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    // pop
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.pop();

    println!("{:?}", v);

    match v.pop() {
        Some(val) => println!("pop: {:?}", val),
        None => println!("vec is empty"),
    }
    match v.pop() {
        Some(val) => println!("pop: {:?}", val),
        None => println!("vec is empty"),
    }
    match v.pop() {
        Some(val) => println!("pop: {:?}", val),
        None => println!("vec is empty"),
    }

    // slice
    let v = vec![1, 2, 3, 4, 5];
    let s = &v[0..2];

    let mut v = vec![1, 2, 3, 4, 5];
    let s = &mut v[0..2];
}
