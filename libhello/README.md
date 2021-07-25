# Libhello

Rustでライブラリを作成するための練習台。  
実行例は、[examples](examples/)を参照。

## API
### activations
[src/activations.rs](src/activations.rs)

活性化関数。スカラ値のみだから今後、`ndarray`とかを使って配列対応したい。

* Sigmoid
* ReLU

### utils
なんか色々

* add
* sub


## Example

```rust
use libhello::utils::math;
use libhello::activations::structs;
use libhello::activations::structs::Activation;


fn main() {
    let a: i32 = 10;
    let b = 10;
    println!("ans: {}", math::add(a, b));

    let activation = structs::Sigmoid::new();
    let inputs: f32 = 0.05;
    println!("Sigmoid: out {}", activation.forward(inputs));
}
```
