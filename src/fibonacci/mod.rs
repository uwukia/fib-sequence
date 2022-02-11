use number::Number;
mod number;

pub use get::nth;
mod get;

pub struct FibonacciSequence {
    prev: Number,
    curr: Number,
}

impl FibonacciSequence {
    pub fn new() -> Self {
        Self { prev: Number::from(0), curr: Number::from(1) }
    }
}

impl Iterator for FibonacciSequence {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let next = format!("{}", self.prev);
        let new_curr = &self.prev + &self.curr;
        self.prev = std::mem::replace(&mut self.curr, new_curr);
        Some(next)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn fibonacci_sequence() {
        for (index, value) in super::FibonacciSequence::new().take(5001).enumerate() {
            println!("f({:02}) = {}", index, value);
        }
    }
}