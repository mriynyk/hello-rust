#![allow(unused)]

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

struct Point3d(f32, f32, f32);

struct Empty;

#[derive(Debug)]
struct Circle {
    center: Point,
    radius: u32,
}

fn main() {
    let p = Point { x: 1.0, y: -2.1 };

    println!("Point (x - {}, y - {})", p.x, p.y);

    let p = Point3d(1.3, -1.5, 2.0);

    println!("Point3d {}, {}, {}", p.0, p.1, p.2);

    let empty = Empty;

    let circle = Circle {
        center: Point { x: 1.1, y: -2.0 },
        radius: 3,
    };

    println!("{:#?}", circle);

    let x = 1.0;
    let y = 1.0;

    let p = Point { x, y };

    let p0 = Point { x: 1.0, y: 1.0 };
    let p1 = Point { x: 1.1, ..p };

    println!("{:#?}", p1);

    let mut p = Point { x: 0.0, y: 0.0 };
    p.x += 0.1;
    p.y += 0.3;

    println!("{:#?}", p);
}
