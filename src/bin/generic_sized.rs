#![allow(unused)]

fn f<T: Sized>(x: T) {}
fn g<T: ?Sized>(x: &T) {}

trait A {}

impl A for u32 {}

fn d(x: Box<dyn A>) {}

fn main() {
    // Examples
    // Primitive types
    let (i, x, b): (i32, f64, bool) = (1, 1.0, true);

    f(i);
    f(x);
    f(b);

    struct S {
        i: i32,
        j: i32,
    }

    let s = S { i: 1, j: 1 };

    f(s);

    let arr: [i32; 4] = [0; 4];

    f(arr);

    f(&arr);

    // ?Sized
    let slice: &[i32] = &[1, 2, 3];
    g(slice);

    let s: &str = "hello";
    g(s);

    let v: Box<dyn A> = Box::new(1u32);
    g(&v);
}
