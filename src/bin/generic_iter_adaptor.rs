#![allow(unused)]

use std::collections::{HashMap, HashSet};

fn main() {
    // map

    let vals: Vec<i32> = vec![1, 2, 3];

    let mut data: Vec<i32> = vec![];

    for v in vals {
        data.push(v * 2);
    }

    println!("{:?}", data);

    let vals: Vec<i32> = vec![3, 4, 5, 3, 4, 5];

    let data: Vec<i32> = vals.iter().map(|x| x * 2).collect();

    println!("{:?}", data);

    // Vec -> HashSet

    let hash_set: HashSet<i32> = vals.iter().map(|x| x * 2).collect();

    println!("{:?}", hash_set);

    // filter

    let vals: Vec<i32> = vec![3, 4, 5, 3, 4, 5];

    let mut data: Vec<i32> = vec![];

    for v in vals {
        if v != 3 {
            data.push(v);
        }
    }

    println!("{:?}", data);

    let vals: Vec<i32> = vec![3, 4, 5, 3, 4, 5];

    let data: Vec<i32> = vals.into_iter().filter(|x| *x != 3).collect();

    println!("{:?}", data);

    // zip

    let keys: Vec<String> = vec!["a", "b", "c", "d"]
        .iter()
        .map(|x| x.to_string())
        .collect();

    let vals: Vec<u32> = vec![1, 2, 3];

    let zipped: Vec<(String, u32)> = keys.into_iter().zip(vals).collect();

    println!("{:?}", zipped);

    // zip -> HashMap

    let hash_map: HashMap<String, u32> = zipped.into_iter().collect();

    println!("{:?}", hash_map);

    // fold

    let vals: Vec<i32> = vec![3, 4, 5, 3, 4, 5];

    let sum = vals.iter().fold(0, |a, x| a + x);

    println!("{:?}", sum);
}
