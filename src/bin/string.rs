#![allow(unused)]

fn main() {
    let msg: String = String::from("Hello rust!");
    let len: usize = msg.len();
    println!("msg: {msg}");
    println!("len: {len}");

    let msg: String = String::from("Hello rust!");
    let s: &str = &msg[0..5];
    let len : usize = s.len();
    println!("s: {s}");
    println!("len: {len}");

    let hello: &str = "hello rust";
    
    let s: &str = r#"
        {
            "a": 1,
            "b": { "c": 2 },
            "d": 3
        }
    "#;
    println!("{s}");

    let msg: String = String::from("Hello rust!!!");
    let s: &str = &msg;
    println!("{s}");

    let mut msg: String = "Hello Rust".to_string();
    msg += "!";
    println!("msg: {msg}");

    let lang = "Rust";
    let emoji = ">_<";
    let msg = format!("Hello {lang} {emoji}");
    println!("msg: {msg}");
}
