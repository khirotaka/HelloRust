fn main() {
    let a = 10;
    let b = 20;

    // (a < b) とすると警告が出る
    if a < b {
        println!("a < b");
    }
    else if a == b {
        println!("a == b");
    }
    else {
        println!("a > b");
    }

    // Pythonみたいに if の結果を変数に代入することができる。
    let x = if a < b { 1 } else { 0 };
    println!("x is {}", x);

    println!("-------------");

    let v = vec![10, 20, 30, 40, 50,];
    print!("v is ");
    for i in &v {
        // この i はあくまで参照
        print!("{} ", i);
        let _x: i32 = *i;
    }
    println!();

    print!("v is ");

    for i in v.iter() {
        print!("{} ", i);
        let _x: i32 = *i;   // これも参照外しが必要
    }
    println!();

    println!("Loop ");
    let mut i = 0;

    loop {
        if i >= 10 {
            break;
        }
        print!("{} ", i);
        i += 1;
    }
    println!();

    // #[] はアトリビュートと呼ばる。
    #[derive(Debug)]
    enum LANG {
        JAPANESE = 81,
        ENGLISH = 44,
        CHINESE = 86,
        FRENCH = 33,
    }

    let lang = LANG::JAPANESE;
    // println!("lang is {:?}", lang as i32);

    let m = match lang {
        LANG::JAPANESE => "日本語",
        LANG::ENGLISH => "英語",
        LANG::CHINESE => "中国語",
        LANG::FRENCH => "フランス語",
    };
    println!("lang is {}", m);

    let lang = LANG::ENGLISH;

    let m = match lang {
        LANG::JAPANESE => "日本語",
        _ => "日本語以外",
    };
    println!("lang is {}", m);


    // let x = Some(10);
    let x = None;
    let v = match x {
        Some(i) => i,
        None => -1,
    };
    println!("v is {} ", v);
}

