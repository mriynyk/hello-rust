#![allow(unused)]

fn main() {
    // Borrow immutable reference &T

    let s = "rust".to_string();
    let fun = || println!("borrow: {}", s);
    fun();
    println!("main: {}", s);

    // Borrow mutable reference &mut T

    let mut s = "rust".to_string();
    let mut fun = || s += "!";
    fun();
    println!("main: {}", s,);

    // Take ownership of value String
    let s = "rust".to_string();

    let fun = move || println!("move: {}", s);

    fun();
    fun();

    // S can not be used since it was moved
    // println!("main: {}", s);

    // Take ownership of value i32
    let val = 1;

    let fun = move || println!("move: {}", val);

    fun();
    fun();

    // Val still can be used since i32 impl Copy trait, fun has its own value
    println!("main: {}", val);

    // Take ownership of value mut String
    let mut s = "rust".to_string();

    let mut fun = move || {
        s += "!";
        println!("move: {}", s);
    };

    fun();
    fun();

    // We can move inside fun, but we can not use outside fun
    // println!("main: {}", s);

    // Take ownership of value mut i32
    let mut val = 1;

    let mut fun = move || {
        val += 1;
        println!("move: {}", val);
    };

    fun();
    fun();

    // Val still can be used since i32 impl Copy trait, fun has its own value
    println!("main: {}", val);

    // Experiments
    fn counter_fn() -> impl FnMut() -> i32 {
        let mut val = 1;

        move || {
            val += 1;
            println!("count: {}", val);

            val // copy
        }
    }

    let mut counter = counter_fn();
    counter();
    let v = counter();
    println!("counter {}", v);

    let mut counter = counter_fn();
    counter();
    counter();
}
