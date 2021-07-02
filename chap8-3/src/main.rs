/*
enum Result<T, E> {
    Ok(T),
    Err(E)
}
*/

use std::num::ParseIntError;
type Result<T> = std::result::Result<T, ParseIntError>;

fn half_number(s: &str) -> Result<i32> {
    match s.parse::<i32>() {
        Ok(n) => Ok(n / 2),
        Err(e) => Err(e)
    }
}

fn half_number_v2(s: &str) -> Result<i32> {
    s.parse::<i32>().map(|x| x / 2)
}

fn half_number_v3(s: &str) -> std::result::Result<i32, &str> {
    match s.parse::<i32>() {
        Ok(n) => Ok(n / 2),
        Err(err) => Err("実行エラー: これは数値であありません。"),
    }
}

fn main() {
    let r = "100".parse::<i32>();
    match r {
        Ok(n) => println!("value: {}", n),
        Err(e) => println!("error: {:?}", e),
    }

    match half_number("100") {
        Ok(t) => println!("value: {}", t),
        Err(e) => println!("Error: {:?}", e),
    }

    match half_number_v2("200") {
        Ok(t) => println!("value: {}", t),
        Err(e) => println!("error: {:?}", e),
    }

    match half_number_v3("xxx") {
        Ok(n) => println!("Ok: {}", n),
        Err(n) => println!("Error: {:?}", n),
    }

    let n = "100".parse::<i32>()
        .expect("これは数値であありません");

    println!("n is {}", n);
    // let n = "xxx".parse::<i32>().expect("これは数値ではありません");
    // println!("n is {}", n);
}
