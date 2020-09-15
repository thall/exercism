pub fn square(s: u32) -> u64 {
    match s {
        1 => 1,
        n @ 2..=64 => (2..=n).into_iter().fold(1, |acc, _| acc * 2),
        _ => panic!("Square must be between 1 and 64"),
    }
}

pub fn total() -> u64 {
    (1..65).map(|x| square(x)).sum()
}
