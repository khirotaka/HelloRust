# ループ

## `while` と `loop`
Rustで無限ループをするためには、 `while` と `loop` の2つの方法がある。

```rust
fn main() {
    while true {
        println("a");
    }
}
```

という 色々な言語で使われる方法に加えて、


```rust
fn main() {
    loop {
        println("a");
    }
}
```

という無限ループ専用の方法もある。後者の方が条件判定をしない分、ちょっとだけ早いらしい

# オプション型の定義

```rust
enum Option<T> {
    Some(T),
    None,
}
```
