#![allow(unused)]

trait Color {
    fn get(&self) -> String;
}

trait Rectangle {
    fn get(&self) -> (i32, i32, u32, u32);
}

struct Square {
    color: String,
    top: i32,
    left: i32,
    size: u32,
}

impl Color for Square {
    fn get(&self) -> String {
        self.color.clone()
    }
}

impl Rectangle for Square {
    fn get(&self) -> (i32, i32, u32, u32) {
        (self.top, self.left, self.size, self.size)
    }
}

fn main() {
    let square = Square {
        top: -2,
        left: 3,
        size: 7,
        color: "red".to_string(),
    };

    let color = Color::get(&square);
    let (top, left, width, height) = Rectangle::get(&square);

    println!("color: {color}, top: {top}, left: {left}, width: {width}, height: {height}")
}
