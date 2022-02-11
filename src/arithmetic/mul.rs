use std::iter;
use crate::arithmetic::{add, BASE};
use super::remove_leading_zeros;

pub fn mul(a: &[u32], b: &[u32]) -> Vec<u32> {
    long_mult(a, b)
}

fn long_mult(a: &[u32], b: &[u32]) -> Vec<u32> {
    let capacity = (a.len() * b.len()) + 1;
    let mut result: Vec<u32> = Vec::with_capacity(capacity);

    for (index, &y) in b.iter().enumerate() {
        let mut multiplication: Vec<u32> = Vec::with_capacity(a.len() + index + 2);

        multiplication.extend(iter::repeat(0u32).take(index));
        multiplication.append(&mut single_mult(a, y));

        result = add(&result, &multiplication);
    }

    remove_leading_zeros(&mut result);
    result
}

fn single_mult(slice: &[u32], y: u32) -> Vec<u32> {

    let mut result: Vec<u32> = Vec::with_capacity(slice.len() + 1);

    let mut carry: u32 = 0;
    for &x in slice.iter() {
        let mul = full_multiplier(x, y, carry);
        carry = mul.1;
        result.push(mul.0);
    }
    result.push(carry);

    remove_leading_zeros(&mut result);
    result
}

// returns (result, carry)
fn full_multiplier(x: u32, y: u32, carry: u32) -> (u32, u32) {
    let base = BASE as u64;
    let mul  = (x as u64) * (y as u64) + (carry as u64);

    let div = mul / base;
    ((mul - (base * div)) as u32, div as u32)
}

#[cfg(test)]
mod tests {

    use rand::Rng;
    use std::time;

    #[test]
    fn single_mult() {
        let mut vec = vec![1u32];
        for n in 2..50u32 {
            vec = super::single_mult(&vec, n);
            println!("{}! = {}", n, crate::arithmetic::display(&vec));
        }
    }

    #[test]
    fn long_mult() {
        let mut vec = vec![7u32];
        let mut pow = 1;
        for _ in 0..10 {
            pow *= 2;
            vec = super::long_mult(&vec, &vec);
            println!("7^{} = {}", pow, crate::arithmetic::display(&vec));
        }
    }

    #[ignore]
    #[test]
    fn bench_long_mult() {
        let mut rng = rand::thread_rng();
        let mut value = vec![rng.gen_range(1..super::BASE)];

        let chunk = 10;
        let size  = 200;

        let mut lengths: Vec<(u32, u128)> = vec![(0, 0); size];
        while value.len() < (chunk * size) {
            let index = value.len() / chunk;

            let now = time::Instant::now();
            let result = super::long_mult(&value, &value);
            let elapsed = now.elapsed().as_nanos();

            assert!(crate::arithmetic::valid(&result));

            let (count, time) = lengths[index];
            lengths[index] = (count+1, time+elapsed);

            value.push(rng.gen_range(1..super::BASE));
        }

        for (length, &(count, time)) in lengths.iter().enumerate() {
            println!("[{:3}] {:+.3e}ns (from {})", length, (time as f64) / (count as f64), count)
        }
    }
}