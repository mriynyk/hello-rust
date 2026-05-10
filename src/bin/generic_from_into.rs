#![allow(unused)]

use std::convert::{From, Into};

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

// (u32, u32) -> Point
impl From<(u32, u32)> for Point {
    fn from(value: (u32, u32)) -> Self {
        Point {
            x: value.0,
            y: value.1,
        }
    }
}

// // Point -> (u32, u32)
// impl Into<(u32, u32)> for Point {
//     fn into(self) -> (u32, u32) {
//         (self.x, self.y)
//     }
// }

fn main() {
    let t: (u32, u32) = (15, 9);
    let p = Point::from(t);

    println!("point: {:?}", p);

    let t: (u32, u32) = (15, 9);

    let p: Point = t.into();

    println!("tuple: {:?}", t);

    let test: String = "Rust!!!".into();

    println!("test: {:?}", test);
}
