use std::option::Option::Some;

fn main() {
    let array = [1, 2, 3, 4, 5];
    println!("array[0] is {}", array[0]);
    println!("array[1] is {}", array[4]);

    println!("array.len() is {}", array.len());

    // let a: [型名; 要素数]
    let array: [u8; 5] = [0x10, 0x20, 0x30, 0x40, 0x50];
    println!("array[0] is {}", array[0]);
    println!("array[4] is {}", array[4]);
    println!("array.len() is {}", array.len());

    let a = [1u8, 2u8, 3u8, 4u8];
    unsafe {
        // 配列から特定の型に変換するときは、 std::mem::transmute関数を使う.
        // これを使うには unsafe で囲む必要がある。
        let b = std::mem::transmute::<[u8; 4], u32>(a);
        println!("b is {:X}", b);
    }

    let a: u32 = 0x11223344;
    unsafe {
        let b = std::mem::transmute::<u32, [u8; 4]>(a);
        println!("b[] is {:X} {:X} {:X} {:X}", b[0], b[1], b[2], b[3]);
    }

    let v = vec![1, 2, 3, 4, 5];

    println!("v.get(0) is {:?}", v.get(0));
    println!("v.get(4) is {:?}", v.get(4));
    println!("v.get(0) is {}", v.get(0).unwrap());
    println!("v.get(4) is {}", v.get(4).unwrap());

    println!("----------");

    let a = vec![1, 2, 3];
    let b = vec![4, 5];
    let v = [a, b].concat();
    println!("v.len() is {}", v.len());

    for i in v {
        print!("{} ", i);
    }
    println!();

    let mut v = vec!["one", "two", "three", "four", "five"];
    v.sort();
    let x = v.join(" ");
    println!("x is {}", x);
    v.reverse();
    let x = v.join(" ");
    println!("x is {}", x);

    let v = vec![1, 2, 3, 4, 5];
    print!("for is ");
    for i in &v {
        print!("{} ", i);
    }
    println!();
    print!("for and iter is ");
    for i in v.iter() {
        print!("{} ", i);
    }
    println!();


    println!("---------");

    let v = vec![1, 2, 3, 4, 5];
    println!("by loop");
    let mut p = v.iter();
    loop {
        let x = p.next();
        if x == None {
            break;
        }
        println!("x is {}", x.unwrap());
    }

    println!("by while");
    let mut p = v.iter();
    while let Some(x) = p.next() {
        println!("x is {}", x);
    }

    println!("---------");

    let v = vec![1, 2, 3, 4, 5];
    let lst = v.iter().map(|x| x * 10);
    for i in lst {
        println!("i is {}", i);
    }
}
