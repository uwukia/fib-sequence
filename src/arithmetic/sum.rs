use crate::arithmetic::{DefaultZip, BASE};
use std::cmp;

pub fn add(a: &[u32], b: &[u32]) -> Vec<u32> {
    let capacity = cmp::max(a.len(), b.len()) + 1;
    let mut result: Vec<u32> = Vec::with_capacity(capacity);

    let mut carry = 0u32;
    for (x, y) in DefaultZip::new(a.iter(), b.iter()) {
        let sum = full_adder(x, y, carry);
        result.push(sum.0);
        carry = sum.1;
    }

    if carry > 0 { result.push(carry) }

    result
}

// returns (result, carry)
fn full_adder(x: u32, y: u32, carry: u32) -> (u32, u32) {
    let sum = x + y + carry;

    let div = sum / BASE;
    (sum - (BASE * div), div)
}

#[cfg(test)]
mod tests {

    use std::time;

    #[ignore]
    #[test]
    fn bench() {
        let mut value = vec![7u32];

        let chunk = 100;
        let size  = 400;

        let mut lengths: Vec<(u32, u128)> = vec![(0, 0); size];
        while value.len() < (chunk * size) {
            let index = value.len() / chunk;
            let now = time::Instant::now();
            let new_value = super::add(&value, &value);
            let elapsed = now.elapsed().as_nanos();

            let (count, time) = lengths[index];
            lengths[index] = (count+1, time+elapsed);

            value = new_value
        }

        for (length, &(count, time)) in lengths.iter().enumerate() {
            println!("[{:3}] {:+.3e}ns (from {})", length, (time as f64) / (count as f64), count)
        }
    }
}