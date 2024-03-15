#![allow(unused)]
#![feature(iter_map_windows)]
use crate::custom_error::AocError;

#[tracing::instrument]
pub fn read_input(input: &str) -> Vec<Vec<i32>> {
    return input
        .lines()
        .map(|line| line.split(' ').map(|x| x.parse().unwrap()).collect())
        .collect();
}

#[tracing::instrument]
pub fn extrapolate(input: &Vec<i32>) -> i32 {
    if input.iter().all(|x| *x == 0) {
        return 0;
    }

    let differences: Vec<i32> =
        (*input.windows(2).map(|x| x[1] - x[0]).collect::<Vec<_>>()).to_vec();
    return input.last().unwrap() + extrapolate(&differences);
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let result = read_input(input)
        .iter()
        .map(|x| extrapolate(x) as i32)
        .sum::<i32>();

    return Ok(result.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA_PART1: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("114", process(TEST_DATA_PART1)?);
        Ok(())
    }
}
