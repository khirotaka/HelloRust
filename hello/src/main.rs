fn main() {
    let s = "This is ねこ🐱neko 文字列";
    let mut v: Vec<char> = Vec::new();

    for c in s.chars() {
        v.push(c);
    }

    let v = &v[8..15];
    let mut s = String::new();
    for c in v {
        s.push(*c);
    }
    println!("s is {}", s);
}
