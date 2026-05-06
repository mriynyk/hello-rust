#![allow(unused)]

use std::collections::HashSet;

fn main() {
    let mut set: HashSet<u32> = HashSet::new();

    let inserted = set.insert(1);
    println!("inserted 1: {inserted}");

    let inserted = set.insert(1);
    println!("inserted 2: {inserted}");

    let contains = set.contains(&1);
    println!("contains 1: {contains}");

    let contains = set.contains(&2);
    println!("contains 2: {contains}");
}
