pub fn build_proverb(list: &[&str]) -> String {
    let mut msgs: Vec<String> = Vec::new();
    if list.is_empty() {
        return String::new();
    }

    for i in 1..list.len() {
        msgs.push(format!("For want of a {} the {} was lost.", list[i - 1], list[i]));
    }

    msgs.push(format!("And all for the want of a {}.", list[0]));
    msgs.join("\n")
}
