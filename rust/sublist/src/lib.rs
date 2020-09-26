use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let mut dropped_front = 0;
    let mut dropped_back = 0;
    {
        let mut i = 0;
        let mut j = 0;
        while i < _first_list.len() && j < _second_list.len() {
            if _first_list[i] != _second_list[j] {
                j += 1;
                dropped_front += 1;
            } else {
                i += 1;
                j += 1;
            }
        }
    }

    {
        let mut i = 0;
        let mut j = 0;
        while i < _first_list.len() && j < _second_list.len() {
            if _first_list[_first_list.len() - 1 - i] != _second_list[_second_list.len() - 1 - j] {
                j += 1;
                dropped_back += 1;
            } else {
                i += 1;
                j += 1;
            }
        }
    }

    println!(
        "dropped_front = {}, dropped_back = {}",
        dropped_front, dropped_back,
    );

    match (dropped_front, dropped_back) {
        (0, 0) if _first_list.is_empty() && _second_list.is_empty() => Comparison::Equal,
        (0, 0) if _first_list.is_empty() => Comparison::Sublist,
        (0, 0) if _second_list.is_empty() => Comparison::Superlist,
        (0, 0) => Comparison::Equal,
        (n, m) if n == m && n == _second_list.len() => Comparison::Unequal,
        (_, m) if m == _second_list.len() => Comparison::Unequal,
        (_, _)
            if _second_list[dropped_front..=(_second_list.len() - 1 - dropped_back)]
                .eq(_first_list) =>
        {
            Comparison::Sublist
        }
        _ => Comparison::Unequal,
    }
}
