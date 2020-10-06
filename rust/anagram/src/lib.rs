use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result = HashSet::new();

    let mut swapped = word.to_owned();
    let mut c = vec![0; word.chars().count()];
    let mut i = 0;

    while i < word.chars().count() {
        if c[i] < i {
            swapped = if i % 2 == 0 {
                swap(&swapped, 0, i)
            } else {
                swap(&swapped, c[i], i)
            };

            if swapped.as_str() != word {
                match possible_anagrams
                    .iter()
                    .find(|x| (**x).to_lowercase() == swapped.to_lowercase())
                {
                    Some(p) => result.insert(*p),
                    None => false,
                };
            }

            c[i] = c[i] + 1;
            i = 0;
        } else {
            c[i] = 0;
            i = i + 1;
        }
    }

    result
}

fn swap(word: &str, i: usize, j: usize) -> String {

    let char_i = word.chars().nth(i).unwrap();
    let char_j = word.chars().nth(j).unwrap();

    word.chars().enumerate().map(|(idx, c)| -> char {
        if idx == i {
            char_j
        } else if idx == j {
            char_i
        } else {
            c
        }
    }).collect()
}
