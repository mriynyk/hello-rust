#![allow(unused)]

fn f_fn() -> impl Fn(i32) -> i32 {
    let v: i32 = 0;
    move |x: i32| v + 1
}

fn f_fn_string() -> impl Fn() -> String {
    let s = "rust".to_string();
    move || {
        println!("s: {}", s);
        s.clone()
    }
}

fn f_fn_mut() -> impl FnMut() {
    let mut s = "hello".to_string();
    move || {
        s.push_str(" rust");
        println!("fn mut: {}", s);
    }
}

fn f_fn_once() -> impl FnOnce() -> String {
    let s = "hello".to_string();
    move || {
        println!("fn once: {}", s);
        s
    }
}

fn main() {
    let f = f_fn();
    println!("f(1): {}", f(1));
    println!("f(1): {}", f(1));

    let mut f = f_fn_mut();
    f();
    f();

    let f = f_fn_once();
    f();
}
