use crate::arithmetic::{self, BASE};
use std::{fmt, ops::{Add, Mul}};

pub struct Number {
    value: Vec<u32>
}

impl Add<&'_ Number> for &'_ Number {
    type Output = Number;

    fn add(self, rhs: &'_ Number) -> Self::Output {
        Number { value: arithmetic::add(&self.value, &rhs.value) }
    }
}

impl Mul<&'_ Number> for &'_ Number {
    type Output = Number;

    fn mul(self, rhs: &'_ Number) -> Self::Output {
        Number { value: arithmetic::mul(&self.value, &rhs.value) }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", arithmetic::display(&self.value))
    }
}

impl From<u32> for Number {
    fn from(num: u32) -> Number {
        if num < BASE {
            Number { value: vec![num] }
        } else {
            Number { value: vec![num % BASE, num / BASE] }
        }
    }
}