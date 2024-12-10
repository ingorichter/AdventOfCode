use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

use miette::miette;

type Num = u64;

#[derive(Debug)]
struct Equation {
    result: Num,
    values: Vec<u64>,
}

// Function to parse a single u32
fn parse_u32(input: &str) -> IResult<&str, Num> {
    map_res(digit1, str::parse)(input)
}

// Function to parse a single line
fn parse_line(input: &str) -> IResult<&str, Equation> {
    let (input, (result, values)) =
        separated_pair(parse_u32, tag(": "), separated_list1(space1, parse_u32))(input)?;
    Ok((input, Equation { result, values }))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Equation>> {
    separated_list1(tag("\n"), parse_line)(input)
}

#[tracing::instrument]
fn can_reach_target(equation: &Equation) -> bool {
    let nums = equation.values.clone();
    let target = equation.result;

    // Recursive helper function
    fn backtrack(nums: &[Num], index: usize, current: Num, target: Num) -> bool {
        // Base case: all numbers are used
        if index == nums.len() {
            return current == target;
        }

        // Try adding the current number
        let add_result = backtrack(nums, index + 1, current + nums[index], target);

        // Try multiplying the current number
        let multiply_result = backtrack(nums, index + 1, current * nums[index], target);

        let concatenated_nums = format!("{}{}", current, nums[index]);
        let concatenated: Num = concatenated_nums.parse::<Num>().unwrap();
        let concat_result = backtrack(nums, index + 1, concatenated, target);

        // Return true if either operation leads to the target
        add_result || multiply_result || concat_result
    }

    // Start backtracking from the first number
    if nums.is_empty() {
        return false; // Edge case: empty list
    }

    backtrack(&nums, 1, nums[0], target)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, parsed_lines) = parse_input(input).map_err(|e| miette!("parse failed {}", e))?;
    let total: Num = parsed_lines
        .iter()
        .filter(|equation| can_reach_target(equation))
        .map(|equation| equation.result)
        .sum();

    // dbg!(&parsed_lines);

    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

        assert_eq!("11387", process(input)?);
        Ok(())
    }
}
