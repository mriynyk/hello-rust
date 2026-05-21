#![allow(unused)]

use std::rc::Rc;

fn main() {
    // #[derive(Debug)]
    // enum List {
    //     Cons(i32, Box<List>),
    //     Nil,
    // }

    // use List::{Cons, Nil};

    // 3 -> 2 -> 1 -> Nil
    //      |
    // 4 <--+

    // let nil = Nil;
    // let one = Cons(1, Box::new(nil));
    // let two = Cons(2, Box::new(one));
    // let a = Cons(3, Box::new(two));
    // // let b = Cons(4, Box::new(two)); // use of moved value: `two`

    #[derive(Debug)]
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    use List::{Cons, Nil};

    // 3 -> 2 -> 1 -> Nil
    //      |
    // 4 <--+

    let list = Rc::new(Cons(2, Rc::new(Cons(1, Rc::new(Nil)))));
    println!("list: {}", Rc::strong_count(&list));

    let a = Cons(3, Rc::clone(&list));
    println!("list: {}", Rc::strong_count(&list));

    {
        let b = Cons(4, Rc::clone(&list));
        println!("list: {}", Rc::strong_count(&list));
    }
    println!("list: {}", Rc::strong_count(&list));

    let mut cur: &List = &a;

    while let Cons(v, tail) = cur {
        print!("{v} -> ");
        // Deref coercion
        // cur = &**tail;
        cur = tail;
    }
}
