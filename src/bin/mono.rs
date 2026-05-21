#![allow(unused)]

struct Point<T> {
    x: T,
    y: T,
}
struct PointU32 {
    x: u32,
    y: u32,
}
struct PointI32 {
    x: i32,
    y: i32,
}

fn get_x<T>(p: Point<T>) -> T {
    p.x
}
fn get_x_u32(p: PointU32) -> u32 {
    p.x
}

fn get_x_i32(p: PointI32) -> i32 {
    p.x
}

fn main() {
    let p0: Point<u32> = Point { x: 0, y: 0 };
    let p1: Point<i32> = Point { x: 0, y: 0 };

    get_x(p0);
    get_x(p1);

    let p0 = PointU32 { x: 0, y: 0 };
    let p1 = PointI32 { x: 0, y: 0 };
}
