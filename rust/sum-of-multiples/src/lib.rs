pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut numbers: Vec<u32> = Vec::new();

    for factor in factors.iter() {
        if *factor == 0 {
            break;
        }
        let mut f: u32 = *factor;

        while f < limit {
            numbers.push(f);
            f += *factor;
        }
    }

    numbers.sort();
    numbers.dedup();
    numbers.iter().fold(0, |acc, v| acc + *v)
}
