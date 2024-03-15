use crate::custom_error::AocError;
use crate::part1::read_input;
use crate::part1::extrapolate;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let result = read_input(input)
        .into_iter()
        .map(|x: Vec<i32>| {
            extrapolate(&x.into_iter().rev().collect::<Vec<_>>())
        })
        .sum::<i32>();

    return Ok(result.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA_PART2: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("2", process(TEST_DATA_PART2)?);
        Ok(())
    }
}