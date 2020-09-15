/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let encoded_without_spaces = plain
        .to_ascii_lowercase()
        .chars()
        .filter(|x| x.is_ascii_alphanumeric())
        .map(|x| atbash(x))
        .collect::<String>();

    let mut encoded = String::new();
    for (i, c) in encoded_without_spaces.chars().enumerate() {
        if i > 0 && i % 5 == 0 {
            encoded.push(' ');
        }
        encoded.push(c)
    }

    encoded
}

fn atbash(x: char) -> char {
    const A_ASCII_CODE: u8 = 97;
    const Z_ASCII_CODE: u8 = 122;

    if x.is_digit(10) {
        x
    } else {
        (A_ASCII_CODE + Z_ASCII_CODE - (x as u8)) as char
    }
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter(|x| !x.is_whitespace())
        .map(|x| atbash(x))
        .collect::<String>()
}
