#![allow(unused)]

fn main() {
    let mut i = 0;

    loop {
        println!("loop {i}");

        if i >= 5 {
            break;
        } else {
            i += 1;
        }
    }

    let mut i = 0;

    while i <= 3 {
        println!("while {i}");
        i += 1;
    }

    for i in 0..=5 {
        println!("for loop {i}");
    }

    let arr = [7, 8, 9];

    for a in arr {
        println!("array {a}");
    }

    let n = arr.len();

    for i in 0..n {
        println!("array {}", arr[i]);
    }

    let v = vec![4, 5, 6];

    for a in v.iter() {
        println!("vector {a}");
    }

    let mut i = 0;

    let res = loop {
        println!("loop {i}");

        if i >= 5 {
            break 99;
        } else {
            i += 1;
        }
    };

    println!("return loop {res}");

    'outer: for i in 0..5 {
        'inner: for j in 0..5 {
            println!("{i}, {j}");

            if i == 1 && j == 3 {
                break 'outer;
            }
        }
    }
}
