#[derive(Debug, PartialEq)]
struct Person {
    id: i32,
    name: String,
    age: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() {
        assert_eq!(3, add(1, 2));
        assert_eq!(10, add(0,10));
        assert_eq!(20, add(10, 10));
    }

    #[test]
    fn test_add_zero() {
        assert_eq!(0, add(0, 0));
    }

    #[test]
    fn test_add_under_zero() {
        assert_eq!(0, add(1, -1));
    }

    #[test]
    fn test_equal_instance() {
        let mut a = Person{id: 100, name: String::from("Taro"), age: 10};
        let b = Person{id: 100, name: String::from("Taro"), age: 10};
        let c = Person{id: 200, name: String::from("Jiro"), age: 30};
        assert_eq!("Taro", a.name);
        assert_eq!("Jiro", c.name);
        println!("a: {:?}", a);
        assert_eq!(a, b);
        a.age += 1;
        println!("a: {:?}", a);
        assert_ne!(a, b);
    }
}

fn add(a: i32, b: i32) -> i32 {
    let ans = a + b;
    ans
}