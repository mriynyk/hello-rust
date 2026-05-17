#![allow(unused)]

fn main() {
    // Function definition
    fn add(x: u32, y: u32) -> u32 {
        x + y
    }

    // Anonymous function
    let fun = |x: u32, y: u32| -> u32 { x + y };

    // Type are inferred
    let f = |x, y| x + y;
    let z = f(1, 2);

    let f = |x, y| {
        // more code here
        x + y
    };
    let z = f(1, 2);

    let v = 1;
    let f = |x: u32| x + v; // v can be used inside the fn

    let w = vec![1, 2, 3];
    let w2: Vec<i32> = w.iter().map(|x| x + 2).collect();
}
