#![allow(unused)]

use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, u32> = HashMap::new();

    scores.insert("red".to_string(), 100);
    scores.insert("green".to_string(), 100);

    let green = "green".to_string();

    let val: Option<&u32> = scores.get(&green);
    // let val: Option<&u32> = scores.get("green");
    println!("{:?}", val);

    let val: Option<&u32> = scores.get("yellow");
    println!("{:?}", val);

    scores.insert("green".to_string(), 200);
    let val: Option<&u32> = scores.get("green");
    println!("{:?}", val);

    println!("Upsert");

    scores.insert("blue".to_string(), 200);

    let v: &mut u32 = scores.entry("blue".to_string()).or_insert(0);
    *v += 300;

    let val: Option<&u32> = scores.get("blue");
    println!("{:?}", val);
}
