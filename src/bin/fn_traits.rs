#![allow(unused)]

// fn f_fn<F: Fn() -> ()>(f: F) {
// fn f_fn<T, F: Fn() -> T>(f: F) {
// fn f_fn(f: impl Fn() -> ()) {
// fn f_fn(f: impl Fn()) {
fn f_fn<F: Fn()>(f: F) {
    f();
    f();
}

fn f_fn_mut<F: FnMut()>(mut f: F) {
    f();
    f();
}

fn f_fn_once<F: FnOnce()>(f: F) {
    f();
}

fn main() {
    // Fn (capture by &)
    let s = "hello".to_string();
    let f = || println!("fn: {}", s);
    f_fn(f);
    println!("main: {}", s);

    // FnMut (capture by &mut)
    let mut v = vec![0];
    let mut f = || v.push(0);
    f_fn_mut(f);
    println!("main: {:?}", v);

    // FnOnce (capture by value)
    let v = vec![1, 2, 3];
    let f = move || println!("fn once: {:?}", v);
    f();
    f_fn_once(f);

    let v = 123;
    // The value is cloned to closure
    let f = move || println!("fn once: {:?}", v);
    f_fn_once(f);
    // The closure is cloned
    f_fn_once(f);
}

