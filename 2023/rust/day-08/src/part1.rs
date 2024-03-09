use crate::custom_error::AocError;

use std::collections::HashMap;
use regex::Regex;

#[derive(Debug)]
enum Direction {
    Left,
    Right
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let mut mapping: HashMap<String, (String, String)> = HashMap::new();
    let mut instructions: Vec<Direction> = Vec::new();

    input.split("\n\n").for_each(|chunk| {
        chunk.lines().for_each(|line| {
            match line {
                line if line.contains(" = ") => {
                    let re = Regex::new(r"^(\S+) = \(((\S+), (\S+))\)$").unwrap(); // Assumes lines are in the format "aaa = (bbb, ccc)"

                    if let Some(cap) = re.captures(line) {
                        let key: String = cap[1].to_owned();
                        let left_value: String = cap[3].to_owned();
                        let right_value: String = cap[4].to_owned();

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

    // iterate through all steps
    let instructionslength = instructions.len();
    let mut index = 0;
    let mut steps = 0;
    let mut key = "AAA";
    let mut current_element = mapping.get(key).unwrap();
    while index < instructionslength {
        let instruction = &instructions[index];
        match instruction {
            Direction::Left => {
                key = current_element.0.as_str();
            }
            Direction::Right => {
                key = current_element.1.as_str();
            }
        }

        index += 1;
        steps += 1;
        if key == "ZZZ" {
            break;
        }

        current_element = mapping.get(key).unwrap();
        if index == instructionslength {
            index = 0;
        }
    }

    // dbg!(instructions);
    return Ok(steps.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA_PART1: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    
    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("6", process(TEST_DATA_PART1)?);
        Ok(())
    }
}
