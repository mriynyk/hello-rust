#![allow(unused)]

fn main() {
    let t: (bool, u32, char) = (true, 1, 'c');

    let (a,b,c) = t;

    let (_,b,_) = t;

    let t = ();

    let nested: ((f64, char), (bool, u32, char), ()) = ((1.23, 'a'), (true, 1u32, 'b'), ());

    let t: (bool, u32, char) = (true, 1, 'c');

    println!("t = {}, {}, {}", t.0, t.1, t.2);

    println!("nested = {}, {}", nested.0.0, nested.1.1);
}
