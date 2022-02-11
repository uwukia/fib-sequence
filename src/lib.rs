//! # Smol Fibonacci Sequence Library
//! 
//! There is, quite honestly, no need for documentation. Everything you need to know is in the
//! readme file, but here is a brief summary:
//! 
//! * Running through the sequence
//! 
//! The [Sequence] struct is an iterator:
//! 
//! ```
//! use fib_sequence::Sequence;
//! for n in Sequence::new() { ... }
//! ```
//! 
//! Just be careful, it's an infinite iterator. So make sure to use something like the
//! [take](std::iter::Iterator::take) method to make it finite.
//! 
//! It returns a string representation of the number in base 10. Simply because that's what this
//! library is meant for. Retrieving big fibonacci numbers in decimal.
//! 
//! * The [nth] function
//! 
//! It returns the nth fibonacci number. Please use this if you want to retrieve one or a few
//! numbers, as iterating through a sequence is much slower.
//! 
//! Produces a string, for the same reason as above.
//! 
//! ```
//! println!("{}", fib_sequence::nth(10_000));
//! ```
//! 
//! That's it.

mod arithmetic;

pub use fibonacci::{FibonacciSequence as Sequence, nth};
mod fibonacci;

#[cfg(test)]
mod tests {
    #[test]
    fn nth() {
        println!("{}", super::nth(1_000_000));
    }
}