// 所有権

#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}

fn print_a(a: &Person) {
    println!("print_a is {:?}", a);
}

fn move_a(a: Person) {
    println!("move_a: a is {:?}", a);
}

fn add_age(a: &mut Person) {
    a.age += 1;
}

fn main() {
    let a = Person{name: String::from("masuda"), age: 50};
    print_a(&a);
    println!("main: a is {:?}", a);
    // move_a(a);
    println!("----------");
    let x = &a;
    println!("変数a の場合: {:?}", a);
    println!("変数x の場合: {:?}", x);

    println!("----------");

    let mut a = Person{name:  String::from("masuda"), age: 50};     // ミュータブルな変数を宣言
    println!("a is {:?}", a);
    add_age(&mut a);        // ミュータブルな参照
    println!("a is {:?}", a);

    // Error 例
    let a = Person{name: String::from("masuda"), age: 50};  // 変数 a はイミュータブル
    let mut x = a;              // 変数 a の中身をミュータブルな変数xに移動
    println!("x is {:?}", x);
    add_age(&mut x);                 // だからインクリメントできる
    println!("x is {:?}", x);
    add_age(&mut x);
    // println!("a is {:?}", a);        // 所有権を失った変数の参照

    println!("----------");

    // let a = Person{name: String::from("masuda"), age: 50};
    // let mut x = &a;
    // println!("x is {:?}", x);
    // x.age += 1;                 // イミュータブルな変数の参照だから行くインクリメントできない
    // println!("x is {:?}", x);
    // これを実現するには以下のようにする。
    let mut a = Person{name: String::from("masuda"), age: 50};  // ミュータブルな変数の宣言
    let mut x = &mut a;     // ミュータブルな参照を宣言
    println!("x is {:?}", x);
    x.age += 1;
    println!("x is {:?}", x);
    add_age(&mut x);
    println!("x is {:?}", x);
}
