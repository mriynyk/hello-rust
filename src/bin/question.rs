#![allow(unused)]

use std::f32::consts::E;

fn f1() -> Result<u32, String> {
    println!("f1 called");
    Ok(1)
    // Err("f1 error".to_string())
}

fn f2() -> Result<u32, String> {
    println!("f2 called");
    Ok(2)
    // Err("f2 error".to_string())
}

fn f3() -> Result<u32, bool> {
    println!("f3 called");
    Ok(3)
    // Err(false)
}

fn f_match() -> Result<u32, String> {
    let res_1 = f1();

    let x_1 = match res_1 {
        Ok(x) => x,
        Err(e) => {
            return Err(e);
        }
    };

    let res_2 = f2();

    let x_2 = match res_2 {
        Ok(x) => x,
        Err(e) => {
            return Err(e);
        }
    };

    Ok(x_1 + x_2)
}

fn f_question() -> Result<u32, String> {
    let res_1 = f1()?;
    let res_2 = f2()?;
    let res_3 = match f3() {
        Ok(x) => x,
        Err(e) => {
            return Err("f3 error".to_string());
        }
    };

    Ok(res_1 + res_2 + res_3)
}

fn main() -> Result<(), String> {
    // let res = f1();

    // match res {
    //   Ok(x) => println!("{x}"),
    //   Err(e) => println!("err: {e}"),
    // }

    // let x = f1()?;
    // println!("x = {x}");

    let z = f_question();

    match z {
        Ok(x) => println!("{x}"),
        Err(e) => println!("err: {e}"),
    }

    Ok(())
}
