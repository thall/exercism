/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn_dash_removed: String = isbn.chars().filter(|x| *x != '-').collect();
    if isbn_dash_removed.len() != 10 {
        return false;
    }

    let sum: u32 = isbn_dash_removed
        .chars()
        .enumerate()
        .filter_map(|(i, v)| {
            if i == 9 && v == 'X' {
                Some(10)
            } else {
                v.to_digit(10)
            }
        })
        .enumerate()
        .map(|(i, v)| v * (10 - i as u32))
        .sum();
    sum % 11 == 0
}
