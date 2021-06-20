fn main() {
    // ビット演算
    let a: u8 = 0b1111;
    let b: u8 = 0b0011;

    println!("a & b is {:04b}", a & b);
    println!("a | b is {:04b}", a | b);

    // シフト演算
    let a: u8 = 0x02;
    println!("a << 1 is {}", a << 1);
    println!("a >> 1 is {}", a >> 1);

    str_param("rust");

    let ret = str_param_and_return("rust");
    println!("ret is {}", ret);

    println!("--------------------");

    let v = vec![1, 2, 3, 4, 5];
    println!("sum is {}", vec_param(&v));

    println!("--------------------");

    let v = vec_return(10);
    for i in v {
        print!("{} ", i);
    }

    println!();
    println!("--------------------");

    let mut v = vec![1, 2, 3, 4, 5];
    vec_change(&mut v);
    for i in v {
        print!("{} ", i);
    }
    println!();
    println!("--------------------");
}

fn str_param(s: &str) {
    println!("called str_param. s is {}", s);
}

fn str_param_and_return(s: &str) -> String {
    println!("called str_param_and_return. s is {}", s);
    let ret = format!("hello {} world", s);
    ret
}

fn vec_param(v: &Vec<i32>) -> i32 {
    println!("called vec_param");
    let mut sum = 0;
    for i in v {
        sum += i;
    }
    sum
}

fn vec_return(max: i32) -> Vec<i32> {
    println!("called vec_return");
    let mut v = Vec::new();
    for i in 0..max {
        v.push(i);
    }
    v
}

fn vec_change(v: &mut Vec<i32>) {   // 引数の中身を直接変更するには mut を使う。
    // ベクターの中身を変更する関数
    println!("called vec_chagne");
    for i in v {
        *i = *i * 10;       // Cにおけるポインタと同じように * で参照先
    }
}