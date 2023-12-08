use std::collections::{BTreeMap, HashSet};

use crate::custom_error::AocError;

#[derive(Debug)]
enum Value {
    Empty,
    Symbol(char),
    Digit(char),
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let grid = input.lines().enumerate().flat_map(|(y, line)| {
        line.chars().enumerate().map(move |(x, c)| {
            (
                (y as i32, x as i32),
                match c {
                    '.' => Value::Empty,
                    c if c.is_ascii_digit() => Value::Digit(c),
                    c => Value::Symbol(c)
                }
            )
        })
    }).collect::<BTreeMap<(i32, i32), Value>>();

    // now the BTreeMap is sorted by (y, x)
    let mut numbers: Vec<((i32, i32), String)> = vec![];
    let mut digits: Vec<char> = vec![];
    let mut symbols: HashSet<(i32, i32)> = HashSet::new();
    let mut start_pos_x = 0;
    let mut start_pos_y = 0;
    // line by line
    for ((y, x), value) in grid.iter() {
        if let Value::Digit(d) = value {
            digits.push(*d);
            if digits.len() == 1 {
                start_pos_x = *x;
                start_pos_y = *y;
            }
        } else if let Value::Symbol(_s) = value {
            symbols.insert((*y, *x));

            if !digits.is_empty() {
                numbers.push((
                    (start_pos_y, start_pos_x),
                    digits.iter().collect::<String>(),
                ));
                digits = vec![];
            }
        } else if !digits.is_empty() {
            numbers.push((
                (start_pos_y, start_pos_x),
                digits.iter().collect::<String>(),
            ));
            digits = vec![];
        }
    }

    // dbg!(&numbers);

    let mut part_numbers: Vec<i32> = vec![];
    for ((y, x), value) in numbers.iter() {
        let mut coordinates: HashSet<(i32, i32)> = HashSet::new();

        let str_len = (value.len() + 1) as i32;
        for i in 0..=str_len {
            coordinates.insert((*y - 1, *x - 1 + i));
            coordinates.insert((*y    , *x - 1 + i));
            coordinates.insert((*y + 1, *x - 1 + i));
        }

        if !coordinates.is_disjoint(&symbols) {
            part_numbers.push(value.parse::<i32>().unwrap());
        }
    }

    let total: i32 = part_numbers.iter().sum();

    return Ok(total.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA_PART1: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("4361", process(TEST_DATA_PART1)?);
        Ok(())
    }
}
