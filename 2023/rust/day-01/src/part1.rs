use crate::custom_error::AocError;

pub fn extract_digits(line: &str) -> Vec<char> {
    return line.chars().filter(|c| c.is_ascii_digit()).collect()
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut sum = 0;

    input.lines().for_each(|line| {
        let digits = extract_digits(line);
        let first = digits.first();
        let last = digits.last();
        let number = format!("{}{}", first.unwrap(), last.unwrap());
        sum += number.parse::<u32>().unwrap();
    });
    return Ok(sum.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA_PART1: &str = "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet";

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("142", process(TEST_DATA_PART1)?);
        Ok(())
    }
}
