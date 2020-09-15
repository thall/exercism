use std::convert::TryInto;

pub fn nth(n: u32) -> u32 {
    (2..).filter(|p| is_prime(*p)).nth(n as usize).unwrap()
}

fn is_prime(number: u32) -> bool {
    if number <= 1 {
        return false;
    }
    if number == 2 {
        return true;
    }
    if number % 2 == 0 {
        return false;
    }

    (3..=(number as f32).sqrt() as u32)
        .step_by(2)
        .all(|i| number % i != 0)
}
