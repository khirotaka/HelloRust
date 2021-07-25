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
