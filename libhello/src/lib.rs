pub mod utils;
pub mod activations;

pub mod greet {
    pub fn hello() {
        println!("Hello!!");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


