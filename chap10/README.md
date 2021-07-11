# 所有権

## 数値やタプル
コピーされる特殊な例。それ以外は

```rust
let x = a;
```
のようにすると所有権が新しい方に移動する。

しかし例外があり、数値やタプルは値がコピーされる。

```rust

let a: i32 = 100;
let x: i32 = a;
let y: i32 = a;
```

このコードは正常に動作する。

タプルを使った例はこちら

```rust
let a: (i32, &str) = (100, "hello!");
// 整数と文字列
let x = a;
let y = a;
```