use std::cmp::Ordering;
use std::io;

pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}