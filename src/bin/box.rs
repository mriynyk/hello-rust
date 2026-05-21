#![allow(unused)]

use std::{fs::File, io::Read};

fn read_file(path: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut data = String::new();
    file.read_to_string(&mut data);
    let num: i32 = data.parse()?;
    Ok(num)
}

#[derive(Debug)]
struct Tree {
    val: i32,
    left: Option<Box<Tree>>,
    right: Option<Box<Tree>>,
}

fn main() {
    let i: i32 = 1;
    let b: Box<i32> = Box::new(i);
    let v = *b;

    let tree = Tree {
        val: 1,
        left: Some(Box::new(Tree {
            val: 2,
            left: None,
            right: Some(Box::new(Tree {
                val: 3,
                left: None,
                right: None,
            })),
        })),
        right: Some(Box::new(Tree {
            val: 4,
            left: None,
            right: None,
        })),
    };

    println!("{tree:#?}");
    println!(
        "tree.left.right.val: {}",
        tree.left.unwrap().right.unwrap().val
    );
}
