pub mod funcs {
    use num::Float;

    pub fn sigmoid<T: Float>(x: T) -> T {
        num::one::<T>() / (num::one::<T>() + (-x).exp())
    }

    pub fn relu<T: Float>(x: T) -> T {
        if x > num::zero::<T>() {
            x
        }
        else {
            num::zero::<T>()
        }
    }
}

#[cfg(test)]
mod tests_funcs {
    use crate::activations::funcs;

    #[test]
    fn test_sigmoid() {
        let a: f32 = 0.05;
        let b = funcs::sigmoid(a);
        assert_eq!(0.51249737, b)
    }

    #[test]
    fn test_relu_zero() {
        let a: f32 = 0.0;
        let b = funcs::relu(a);
        assert_eq!(a, 0.0);
    }

    #[test]
    fn test_relu_not_zero() {
        let a: f32 = 1.0;
        let b = funcs::relu(a);
        assert_eq!(a, b);
    }
}

pub mod structs {
    use num::Float;
    use crate::activations::funcs;

    pub trait Activation {
        fn forward<T: Float>(&self, x: T) -> T;
    }


    pub struct Sigmoid {
    }
    /// Sigmoid関数
    /// 入力値(スカラ) に対応する値を返す
    /// ```rust
    /// use libhello::activations::structs::{Sigmoid, Activation};
    /// let act = Sigmoid::new();
    /// let input: f32 = 0.05;
    /// let out = act.forward(input);
    /// ```
    ///
    impl Sigmoid {
        pub fn new() -> Sigmoid {
            Sigmoid{}
        }
    }
    impl Activation for Sigmoid {
        fn forward<T: Float>(&self, x: T) -> T {
            funcs::sigmoid(x)
        }
    }

    pub struct ReLU {
    }
    impl ReLU {
        pub fn new() -> ReLU {
            ReLU{}
        }
    }
    impl Activation for ReLU {
        fn forward<T: Float>(&self, x: T) -> T {
            funcs::relu(x)
        }
    }
}
#[cfg(test)]
mod tests_structs {
    use crate::activations::structs::Activation;

    #[test]
    fn test_sigmoid_struct() {
        use crate::activations::structs::Sigmoid;
        let act = Sigmoid::new();
        let a: f32 = 0.05;
        let b = act.forward(a);
        assert_eq!(0.51249737, b)
    }

    #[test]
    fn test_relu_under_zero_struct() {
        use crate::activations::structs::ReLU;
        let act = ReLU::new();
        let a: f32 = -5.0;
        let b = act.forward(a);
        assert_eq!(b, 0.0);
    }

    #[test]
    fn test_relu_not_zero_struct() {
        use crate::activations::structs::ReLU;
        let act = ReLU::new();
        let a: f32 = 1.0;
        let b = act.forward(a);
        assert_eq!(b, 1.0);
    }
}