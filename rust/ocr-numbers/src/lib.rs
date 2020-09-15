// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

use crate::Error::{InvalidRowCount, InvalidColumnCount};
use core::num::FpCategory::Zero;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

const ZERO: [[char; 3]; 4] = [
    [' ', '_', ' '],
    ['|', ' ', '|'],
    ['|', ' ', '|'],
    [' ', '_', '|'],
];

const ONE: [[char; 3]; 4] = [
    [' ', ' ', ' '],
    [' ', ' ', '|'],
    [' ', ' ', '|'],
    [' ', ' ', ' '],
];
const TWO: [[char; 3]; 4] = [
    [' ', '_', ' '],
    [' ', '_', '|'],
    ['|', '_', ' '],
    [' ', ' ', ' '],
];
const TRHEE: [[char; 3]; 4] = [
    [' ', '_', ' '],
    [' ', '_', '|'],
    [' ', '_', '|'],
    [' ', ' ', ' '],
];
const FOUR: [[char; 3]; 4] = [
    [' ', ' ', ' '],
    ['|', '_', '|'],
    [' ', ' ', '|'],
    [' ', ' ', ' '],
];
const FIVE: [[char; 3]; 4] = [
    [' ', '_', ' '],
    ['|', '_', ' '],
    [' ', '_', '|'],
    [' ', ' ', ' '],
];
const SIX: [[char; 3]; 4] = [
    [' ', '_', ' '],
    ['|', '_', ' '],
    ['|', '_', '|'],
    [' ', ' ', ' '],
];
const SEVEN: [[char; 3]; 4] = [
    [' ', '_', ' '],
    [' ', ' ', '|'],
    [' ', ' ', '|'],
    [' ', ' ', ' '],
];
const EIGHT: [[char; 3]; 4] = [
    [' ', '_', ' '],
    ['|', '_', '|'],
    ['|', '_', '|'],
    [' ', ' ', ' '],
];
const NINE: [[char; 3]; 4] = [
    [' ', '_', ' '],
    ['|', '_', '|'],
    [' ', '_', '|'],
    [' ', ' ', ' '],
];

const DIGIST: [[[char; 3]; 4]; 10] = [ZERO, ONE, TWO, TRHEE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE];


pub fn convert(input: &str) -> Result<String, Error> {
    let s = sanity_check_input(input);
    if s.is_err() {
        Err(s.unwrap_err())
    } else {
        let x = parse_input(input);

        let y = x.or(Some("?".to_string()));
        Ok(y.unwrap())
    }
}

fn sanity_check_input(input: &str) -> Result<usize, Error> {
    let rows = input.lines().count();
    if rows != 4 {
        return Err(InvalidRowCount(rows));
    }

    let columns = input.lines().nth(0).unwrap().chars().count();

    if columns % 3 != 0 {
        return Err(InvalidColumnCount(columns));
    }

    println!("Size {} {} Digits: {}", rows, columns, columns / 3);

    return Ok(columns / 3);
}

fn parse_input(input: &str) -> Option<String> {
    let mut array: Vec<Vec<char>> = vec![];
    for (row, line) in input.lines().enumerate() {
        for (column, c) in line.chars().enumerate() {
            array[row][column] = c
        }
    }

    DIGIST.iter().enumerate().find_map(|(i, d)| if *d == array {
        Some(i.to_string())
    } else {
        None
    })
}
