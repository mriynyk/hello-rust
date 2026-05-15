#![allow(unused)]

fn main() {
    let vals: Vec<i32> = vec![1, 2, 3];

    // By default vals.into_iter() if you need value itself (move)
    // vals.iter() if you do not need value itself (borrow)
    for v in vals.iter() {
        println!("{}", v);
    }

    let mut vals: Vec<i32> = vec![1, 2, 3];

    // By default vals.into_iter() (mut) - move
    // vals.iter_mut() - borrow
    for v in vals.iter_mut() {
        *v += 1;
        println!("{}", v);
    }
}
