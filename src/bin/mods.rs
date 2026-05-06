#![allow(unused)]

// mod foo {
//     pub fn print() {
//         println!("foo");
//     }
// }

// mod my {
//     use super::foo;

//     pub fn call_foo() {
//         foo::print();
//     }

//     pub fn print() {
//         f();
//         println!("my");
//     }

//     fn f() {
//         a::print();
//     }

//     pub mod a {
//         #[derive(Debug)]
//         pub struct S {
//             pub id: u32,
//             pub name: String,
//         }

//         pub fn print() {
//             println!("a");
//         }

//         use super::super::foo;

//         pub fn call_foo() {
//             foo::print();
//         }

//     }
// }

use hello_rust::my;

fn main() {
    my::print();
    my::a::print();

    let s = my::a::S {
        id: 1,
        name: "Vovchik".to_string(),
    };

    println!("{:#?}", s);

    my::call_foo();
    my::a::call_foo();
}
