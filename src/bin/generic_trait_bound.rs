#![allow(unused)]

fn f<T: std::fmt::Debug>(t: T) {
    println!("{:?}", t);
}

trait A {}
trait B {}
trait C {}

impl A for u32 {}
impl B for u32 {}
impl C for u32 {}
impl A for i32 {}

fn c<T: A>(x: T) {}
fn m<T: A + B>(x: T) {}
fn w<T, U>(x: T, y: U)
where
    T: A + B,
    U: B + C,
{
}

// diff between impl trait syntax and trait bound
// x and y can be diff type
fn k(x: impl A, y: impl A) {}
fn g<T: A>(x: T, y: T) {}
fn h<T: A, U: A>(x: T, y: U) {}

fn main() {
    let u: u32 = 19;
    let i: i32 = -19;
    let f: f32 = 1.0;

    c(u);
    c(i);
    // c(f); does not impl A

    m(u);
    // m(i); does not impl B

    w(u, u);
    // w(u, i); i does not impl B + C

    k(u, i); // both impl trait A
    g(u, u); // the same type
    g(i, i);
    // g(u, i); diff types

    h(u, i); // now works
    h(i, u);
}
