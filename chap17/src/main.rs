fn main() {
    println!("use map");
    let a = [1, 2, 3, 4, 5];
    let b = a.iter().map(|x| x * x);
    for it in b {
        println!("it is {}", it);
    }

    println!("use filter");
    let a = [1, 2, 3, 4, 5];
    let b = a.iter().filter(|&x| x % 2 == 1);
    for it in b {
        println!("it is {}", it);
    }

    println!("use find");
    let a = [1, 2, 3, 4, 5];
    let b = a.iter().find(|&&x| x == 3);    // && は参照の参照
    let c = b.iter().find(|&&x| x > &10);
    println!("b is {:?}", b);
    println!("c is {:?}", c);

    println!("use max & min");
    let a = [1, 2, 3, 4, 5];
    let max = a.iter().max();
    let min = a.iter().min();

    println!("max is {:?}", max);
    println!("min is {:?}", min);

    println!("use zip");
    let a = [1, 2, 3, 4, 5];
    let b = ["one", "two", "three", "four", "five"];
    let c: Vec<_> = a.iter().zip(b.iter()).collect();
    for (n, w) in c {
        println!("c is {} and {}", n, w);
    }

    let print_name = |name: &str, age: i32| {
        println!("name: {}, age: {}", name,age);
    };
    println!("call closure");
    print_name("Taro", 10);

    let a = [("taro", 20), ("jiro", 19), ("saburo", 11)];
    let b = a.iter().map(|(name, age) | {
        format!("name: {}, age: {}", name, age)
    });

    for it in b {
        println!("{}", it);
    }

    let double = |x| x * 2;
    let triplet = |x| x * 3;
    let a = call_with_one(100, double);
    let b = call_with_one(100, triplet);

    println!("a is {}", a);
    println!("b is {}", b);

    let double = |x| x * 2;
    let v = vec![1, 2, 3, 4, 5];
    let a = call_with_vec(&v, double);
    println!("a is {}", a);
    let sum: usize = v.iter().map(|x|x * 2).sum();
    println!("sum is {}", sum);
}

fn call_with_one<F>(x: usize, func: F) -> usize where F: Fn(usize) -> usize {
    // 関数であることを伝えるためには、 Fn を使う
    func(x)
}


fn call_with_vec<F>(v: &Vec::<usize>, func: F) -> usize where F: Fn(usize) -> usize {
    let mut sum = 0;
    for it in v {
        sum += func(*it);
    }
    sum
}