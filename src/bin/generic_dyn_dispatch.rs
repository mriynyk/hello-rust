#![allow(unused)]

#[derive(Debug)]
struct A {}

#[derive(Debug)]
struct B {}

trait F {
    fn f(&self);
}

impl F for A {
    fn f(&self) {
        println!("{:?}", self);
    }
}

impl F for B {
    fn f(&self) {
        println!("{:?}", self);
    }
}

fn static_dispatch<T: F>(t: &T) {
    t.f();
}

fn dyn_dispatch(t: &dyn F) {
    t.f();
}

fn dyn_box_dispatch(t: Box<dyn F>) {
    t.f();
}

fn main() {
    let obj = A {};

    static_dispatch(&obj);

    let obj = B {};

    static_dispatch(&obj);

    let input = "A";

    let obj: &dyn F = match input {
        "A" => &A {},
        "B" => &B {},
        _ => panic!(),
    };

    dyn_dispatch(obj);

    let obj: Box<dyn F> = match input {
        "A" => Box::new(A {}),
        "B" => Box::new(B {}),
        _ => panic!(),
    };

    dyn_box_dispatch(obj);
}
