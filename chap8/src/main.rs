struct Person {
    id: i32,
    name: String,
    age: i32,
    addr: String,
}

static mut PERSON_ID: i32 = 0;

impl Person {
    fn new(name: &str, age: i32, addr: &str) -> Person {
        let id = unsafe {
            PERSON_ID += 1;
            PERSON_ID
        };
        Person {
            id, name: name.to_string(), age, addr: addr.to_string()
        }
    }
    fn print(&self) {
        println!("{}: {} ({}) in {}", self.id, self.name, self.age, self.addr);
    }

    fn print_t(&self, private: bool) {
        if private {
            println!("{}: {}", self.id, self.name);
        }
        else {
            println!("{}: {} ({}) in {}", self.id, self.name, self.age, self.addr);
        }
    }

    fn to_str(&self) -> String {
        let s = format!("{}: {} ({}) in {}", self.id, self.name, self.age, self.addr);
        s
    }

    fn add_age(&mut self, n: i32) {
        self.age += n;
    }
}

/*
 Rustの構造体は、C言語の構造体とメモリ配置が異なるので、#[repr(C)] を付けることで互換を取る。
*/

struct A {
    id: i32,
}

struct B {
    id1: i32,
    id2: i32,
    id3: i32,
}

struct C {
    id: i32,
    name: String,
}

struct D {
    id: i32,
    name: &'static str,
}

struct E {
    id: i32,
    v: Vec<u8>,
}

struct F {
    id: i32,
    v: [u8; 100],
}

fn main() {
    let mut pa = Person{
        id: 100, name: String::from("Taro"), age: 20, addr: String::from("Tokyo")
    };

    println!("{}: {} ({}) in {}", pa.id, pa.name, pa.age, pa.addr);
    pa.addr = String::from("Osaka");
    println!("{}: {} ({}) in {}", pa.id, pa.name, pa.age, pa.addr);

    println!("------------");
    print_person(&pa);

    // 可変な参照
    add_age(&mut pa);
    print_person(&pa);

    let pa2 = new_person(200, "kato");
    println!("{}: {} ({}) in {}", pa2.id, pa2.name, pa2.age, pa2.addr);

    println!("------------");
    let pa = new_person(100, "masuda");
    let pa2 = new_person(200, "kato");

    let mut people = vec![pa, pa2];
    people.push(new_person(200, "yamada"));
    people.push(new_person(200, "sato"));

    for p in &people {
        print_person(p);
    }

    struct Color(f32, f32, f32);    // フィールド名の無い構造体
    let yellow = Color(1.0, 1.0, 0.0);
    // .0, .1, .2 のようにアクセスする。
    println!("R: {:.2} G: {:.2} B:{:.2}", yellow.0, yellow.1, yellow.2);

    println!("------------");

    println!("A size_of is {}", std::mem::size_of::<A>());
    println!("B size_of is {}", std::mem::size_of::<B>());
    println!("C size_of is {}", std::mem::size_of::<C>());
    println!("D size_of is {}", std::mem::size_of::<D>());
    println!("E size_of is {}", std::mem::size_of::<E>());
    println!("F size_of is {}", std::mem::size_of::<F>());

    let pa = Person{id: 1, name: String::from("masuda"), age: 50, addr: String::from("Tokyo")};
    pa.print();

    pa.print_t(true);
    pa.print_t(false);
    let s = pa.to_str();
    println!("s is {}", s);
    println!("==================");

    let mut pa = Person{id: 1, name: String::from("masuda"), age: 50, addr: String::from("Tokyo")};
    pa.print();
    pa.add_age(1);
    pa.print();

    println!("==================");

    let mut people = Vec::<Person>::new();
    people.push(Person::new("Taro", 10, "Tokyo"));
    people.push(Person::new("Jiro", 20, "Osaka"));
    people.push(Person::new("Saburo", 30, "Hokkaido"));

    for p in &people {
        p.print();
    }

    println!("==================");
    
}

// 可変な参照
fn add_age(pa: &mut Person) {
    pa.age += 1;
}

fn print_person(pa: &Person) {
    println!("{}: {} ({}) in {}", pa.id, pa.name, pa.age, pa.addr);
}

fn new_person(id: i32, name: &str) -> Person {
    let pa = Person{
        id, name: name.to_string(), age: -1, addr: String::from("Unknown")
    };
    pa
}
