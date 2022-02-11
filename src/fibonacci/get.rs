use super::Number;

pub fn nth(n: u32) -> String {

    if n < 3 {
        return String::from(match n {
            0 => "0",
            1 => "1",
            2 => "1",
            _ => unreachable!(),
        })
    }

    let mut prev = Number::from(1u32);
    let mut curr = Number::from(1u32);

    for (index, should_add) in get_path(n).into_iter().enumerate().rev() {
        if should_add {
            let new_curr = &prev + &curr;
            prev = std::mem::replace(&mut curr, new_curr);
        }
        
        if index > 0 {
            let dummy = &prev + &prev;
            prev = &(&prev * &prev) + &(&curr * &curr);
            curr = &curr * &(&curr + &dummy);
        }

    }

    format!("{}", curr)
}

fn get_path(mut input: u32) -> Vec<bool> {
    let mut vec: Vec<bool> = Vec::with_capacity(32);

    while input > 1 {
        vec.push(input % 2 == 1);
        input >>= 1;
    }

    vec
}

#[cfg(test)]
mod tests {
    #[test]
    fn verify_nth() {
        for (index, value) in crate::Sequence::new().enumerate().take(1000) {
            assert_eq!(value, super::nth(index as u32));
        }
    }
}