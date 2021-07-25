use rand::{Rng, SeedableRng};

use libhello::utils::math;
use libhello::activations::structs;
use libhello::activations::structs::Activation;


fn main() {
    let a: i32 = 10;
    let b = 10;
    println!("ans: {}", math::add(a, b));

    let sigmoid = structs::Sigmoid::new();
    let inputs: f32 = 0.05;
    println!("Sigmoid: out {}", sigmoid.forward(inputs));

    let seed: [u8; 32] = [0; 32];   // SeedableRng::from_seed(seed) で 32個の要素が要求されているから
    let mut rng: rand::rngs::StdRng = SeedableRng::from_seed(seed);

    let relu = structs::ReLU::new();
    let inputs: f32 = rng.gen();
    println!("ReLU: out {}", relu.forward(inputs));
}
