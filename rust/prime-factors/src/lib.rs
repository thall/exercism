pub fn factors(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();
    let mut number = n;
    let mut i = 2;

    while i <= number {
        if number % i == 0 {
            number /= i;
            factors.push(i);
        } else {
            i += 1
        }
    }

    factors
}
