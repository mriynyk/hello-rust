#![allow(unused)]

use std::cmp::PartialOrd;

fn swap<A, B>(t: (A, B)) -> (B, A) {
    (t.1, t.0)
}

fn max<T: PartialOrd>(slice: &[T]) -> Option<&T> {
    if slice.len() == 0 {
        return None;
    }

    let mut largest = &slice[0];

    for item in slice {
        if item > largest {
            largest = item
        }
    }

    Some(largest)
}

fn main() {
    let t = (1, 2);
    let s = swap(t);
    println!("{:?}", s);

    let v = vec![1, 2, 3, 4, 5, 6];

    let largest = max(&v);

    println!("largest: {:?}", largest);

    let v = vec!['a', 'b', 'c', 'd'];

    let largest = max(&v);

    println!("largest: {:?}", largest);
}
