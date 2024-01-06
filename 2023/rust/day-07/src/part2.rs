use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    todo!("day 01 - part 1");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA_PART2: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    
    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("5905", process(TEST_DATA_PART2)?);
        Ok(())
    }
}