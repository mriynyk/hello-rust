#![allow(unused)]

fn take_string(s: String) {}
fn borrow_string(s: &String) {}
fn mut_string(s: &mut String) {
  s.push('?');
}

fn borrow_str(s: &str) {}

// fn take_str(s: str) {}
// fn make_str() -> str {}
// fn make_str() -> &str {
//   let s = "";
//   s
// }

// it works
// fn return_str_ref(s: &str) -> &str {
//   s
// }

fn main() {
    // String
    let s = String::from("rust");
    take_string(s);

    let mut s = String::from("rust");
    s += "!";

    let s = String::from("rust");
    borrow_string(&s);
    borrow_str(&s);

    let mut s = String::from("rust"); 
    let s1: &mut String = &mut s;
    mut_string(s1);

    // str

    let s: &str = "hello";
    borrow_str(s);

    let mut s = String::from("rust");
    let r: &mut str = &mut s;
}
