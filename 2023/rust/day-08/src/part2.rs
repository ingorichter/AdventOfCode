use crate::custom_error::AocError;

use std::collections::HashMap;
use regex::Regex;

#[derive(Debug)]
enum Direction {
    Left,
    Right
}

pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

pub fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

#[tracing::instrument]
pub fn count_steps(
    mapping: &HashMap<&str, (&str, &str)>,
    instructions: &Vec<Direction>,
    start_key: &str) -> usize {

    let mut steps = 0;
    let mut current_key: &str = start_key;

    while !current_key.ends_with('Z') {
        let current_value = mapping.get(current_key).unwrap();

        let instruction = &instructions[steps % instructions.len()];
        match instruction {
            Direction::Left => {
                current_key = current_value.0;
            }
            Direction::Right => {
                current_key = current_value.1;
            }
        }

        steps += 1;
    }

    steps
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let mut mapping: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut instructions: Vec<Direction> = Vec::new();

    input.split("\n\n").for_each(|chunk| {
        chunk.lines().for_each(|line| {
            match line {
                line if line.contains(" = ") => {
                    let re = Regex::new(r"^(\S+) = \(((\S+), (\S+))\)$").unwrap(); // Assumes lines are in the format "aaa = (bbb, ccc)"

                    if let Some(cap) = re.captures(line) {
                        let key: &str = cap.get(1).map(|f| f.as_str()).unwrap();
                        let left_value: &str = cap.get(3).map(|f| f.as_str()).unwrap();
                        let right_value: &str = cap.get(4).map(|f|f.as_str()).unwrap();

                        mapping.insert(key, (left_value, right_value));
                    }
                }
                &_ => {
                    line.chars().for_each(|c| {
                        match c {
                          'L' => instructions.push(Direction::Left),
                          'R' => instructions.push(Direction::Right),
                          _ => {}
                        }
                      });
                    }
            }
        })
    });

    // iterate through all keys
    // use every key in each round and check if the key ends with Z
    // store the last key as the first key for next iteration
    // repeat until all the keys end with Z
    // return the number of steps

    let steps: usize = mapping.keys().filter(|key| key.ends_with('A')).map(|k| count_steps(&mapping, &instructions, k)).reduce(lcm).unwrap();

    Ok(steps.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA_PART2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
    
    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("6", process(TEST_DATA_PART2)?);
        Ok(())
    }
}