## vector
vectorの宣言には、 `vec!` を使うか、 `Vec::new()` をして、 `push`する。

vector空値を取得するには、配列を同じように、`[]` を使う方法の他に、`.get()` を使う方法がある。  
後者の場合、オプション型が取得される。


以下のように型を定義しなくても vectorを宣言することができる。

```rust
fn main() {
    let mut v = Vec::new();
    v.push(10);    // 初めて値が代入された段階で肩が決定する。
}
```

## iter
```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    for i in v {
        print!("{} ", i);
    }
}
```

と

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    for i in v.iter() {
        print!("{} ", i);
    }
}
```

は同じ動作をする。  
違いは後者の方は返されるのがオプション型である点。

