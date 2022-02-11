pub const BASE: u32 = 1_000_000_000;

pub use zip::DefaultZip;
mod zip;

pub use sum::add;
mod sum;

pub use mul::mul;
mod mul;

pub use display::display;
mod display;

fn remove_leading_zeros(vec: &mut Vec<u32>) {
    while vec.len() > 1 && vec.iter().last().unwrap() == &0 {
        vec.pop();
    }
}

#[allow(dead_code)]
fn valid(slice: &[u32]) -> bool {
    slice.iter()
    .fold(true, |acc, &cur| acc && (cur < BASE))
}