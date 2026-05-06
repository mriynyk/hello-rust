#![allow(unused)]

#[derive(Debug)]
enum MathError {
    DivByZero,
    Other,
}

fn div(x: u32, y: u32) -> Result<u32, MathError> {
    if y == 0 {
        Err(MathError::DivByZero)
    } else {
        Ok(x / y)
    }
}

fn main() {
    // panic!("crash");

    let arr: [i32; 3] = [1, 2, 3];

    let element: Option<&i32> = arr.get(9);

    match element {
        Some(val) => println!("val = {val}"),
        None => println!("None"),
    }

    let x = 1;
    let y = 0;
    let z = div(x, y);

    match z {
        Ok(val) => println!("val = {val}"),
        Err(err) => println!("err = {:?}", err),
    }
}
