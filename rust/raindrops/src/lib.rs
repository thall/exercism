pub fn raindrops(n: u32) -> String {
    let mut message: String = String::new();

    if n % 3 != 0 && n % 5 != 0 && n % 7 != 0 {
        return n.to_string();
    }

    if n % 3 == 0 {
        message += "Pling"
    }

    if n % 5 == 0 {
        message += "Plang"
    }

    if n % 7 == 0 {
        message += "Plong"
    }

    return message;
}
