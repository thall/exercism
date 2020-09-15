pub fn is_armstrong_number(num: u32) -> bool {
    let digits = number_of_digits(num);

    num == armstrong_sum(num, digits, 0)
}

fn number_of_digits(num: u32) -> u32 {
    if num == 0 {
        return 0;
    }

    1 + number_of_digits(num / 10)
}

fn armstrong_sum(num: u32, digits: u32, acc: u32) -> u32 {
    if num == 0 {
        return acc;
    }

    armstrong_sum(num / 10, digits, acc + (num % 10).pow(digits))
}
