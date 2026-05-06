#![allow(unused)]

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
    Rgba(u8, u8, u8, f32),
    Hex(String),
    Hsl { h: u8, s: u8, l: u8 },
}

fn main() {
    let color: Color = Color::Red;
    let color: Color = Color::Green;
    let color: Color = Color::Rgba(0, 0, 255, 0.5);
    let color: Color = Color::Hex("#ffee11".to_string());
    let color: Color = Color::Hsl { h: 0, s: 1, l: 1 };

    println!("{:?}", color);
    println!("{}", Color::Red == Color::Green);
    println!("{}", Color::Red == Color::Red);

    // Option = Some(11) | None
    let x: Option<i32> = None;
    let x: Option<i32> = Some(-1);
    println!("option: {:?}", x);
    // Result = Ok(10) | Err("Error!")
    let res: Result<u32, String> = Ok(5);
    let res: Result<u32, String> = Err("Error!".to_string());
    println!("result: {:?}", res);
}
