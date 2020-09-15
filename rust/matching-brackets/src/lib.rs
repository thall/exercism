pub fn brackets_are_balanced(string: &str) -> bool {
    let filtered = string.chars()
        .filter(|x| is_bracket(x))
        .collect::<String>();

    if filtered.len() % 2 != 0 {
       return false
    }

    let stack : Vec<char> = Vec::with_capacity(filtered.len());
    let stack = filtered
        .chars()
        .fold(stack, |stack, c| b(c, stack));

    stack.is_empty()
}

fn b(c: char, mut stack: Vec<char>) -> Vec<char> {
    match c {
        '(' | '[' | '{'                                    => {stack.push(c); stack},
        ')' if *(stack.last().unwrap_or(&'_')) == '(' => {stack.pop(); stack},
        ']' if *(stack.last().unwrap_or(&'_')) == '[' => {stack.pop(); stack},
        '}' if *(stack.last().unwrap_or(&'_')) == '{' => {stack.pop(); stack},
        _                                                  => stack,
    }
}

fn is_bracket(c: &char) -> bool {
    match c {
        '(' | '[' | '{' | ')' | ']' | '}' => true,
        _ => false,
    }
}
