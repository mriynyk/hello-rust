#![allow(unused)]

enum Option<T> {
    None,
    Some(T),
}

enum Result<R, E> {
    Ok(R),
    Err(E),
}

struct Point<T = u32> {
    x: T,
    y: T,
}

fn main() {
    let x: Option<u32> = Option::Some(1);
    let x: Option<i32> = Option::Some(-1);

    let res: Result<bool, String> = Result::Ok(true);

    let v: Vec<u32> = vec![1, 2, 3];
    let v: Vec<_> = vec![1, 2, 3];

    let p0: Point = Point { x: 1, y: 1 };
    let p1: Point<i32> = Point { x: 1, y: -1 };
    let p2: Point<_> = Point { x: "2", y: "1" };
}
