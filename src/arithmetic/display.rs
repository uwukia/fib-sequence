pub fn display(slice: &[u32]) -> String {
    slice.iter().rev().enumerate()
    .map(|(index, &num)| {
        if index == 0 {
            format!("{}", num)
        } else {
            format!("{:09}", num)
        }
    })
    .collect::<Vec<String>>().join("")
}

#[cfg(test)]
mod tests {

    use crate::arithmetic::valid;
    use std::time;

    #[ignore]
    #[test]
    fn bench_valid() {
        let mut value = vec![7u32];

        let chunk = 100;
        let size  = 400;

        let mut lengths: Vec<(u32, u128)> = vec![(0, 0); size];
        while value.len() < (chunk * size) {
            let index = value.len() / chunk;
            let new_value = crate::arithmetic::add(&value, &value);

            let now = time::Instant::now();
            assert!(valid(&new_value));
            let elapsed = now.elapsed().as_nanos();

            let (count, time) = lengths[index];
            lengths[index] = (count+1, time+elapsed);

            value = new_value
        }

        for (length, &(count, time)) in lengths.iter().enumerate() {
            println!("[{:3}] {:+.3e}ns (from {})", length, (time as f64) / (count as f64), count)
        }
    }

    #[ignore]
    #[test]
    fn bench_display() {
        let mut value = vec![7u32];

        let chunk = 100;
        let size  = 100;

        let mut lengths: Vec<(u32, u128)> = vec![(0, 0); size];
        while value.len() < (chunk * size) {
            let index = value.len() / chunk;
            let new_value = crate::arithmetic::add(&value, &value);

            let now = time::Instant::now();
            assert!(super::display(&new_value) != "");
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