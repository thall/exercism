pub fn reply(message: &str) -> &str {
    let trimmed = message.trim();
    if trimmed.is_empty() {
        "Fine. Be that way!"
    } else if is_yelling(trimmed) {
        if is_question(trimmed) {
            "Calm down, I know what I'm doing!"
        } else {
            "Whoa, chill out!"
        }
    } else if is_question(trimmed) {
        "Sure."
    } else {
        "Whatever."
    }
}

fn is_yelling(message: &str) -> bool {
    message.chars().filter(|x| x.is_alphabetic()).count() > 0 && // checking count because of Vacuous truth
        message.chars().filter(|x| x.is_alphabetic()).all(|x| x.is_uppercase())
}

fn is_question(message: &str) -> bool {
    message.chars().last().unwrap() == '?'
}