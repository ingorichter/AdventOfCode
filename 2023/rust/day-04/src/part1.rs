use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let mut scratch_card_score = 0;

    input.lines().for_each(|line| {
        let (_card, numbers) = line.split_once(": ").unwrap();
        let (my_nums, winning_nums) = numbers.split_once(" | ").unwrap();
        let my_numbers = my_nums.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let winning_numbers = winning_nums.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();

        let matching_numbers = my_numbers.iter().filter(|x| winning_numbers.contains(x)).collect::<Vec<&u32>>();
        if !matching_numbers.is_empty() {
            scratch_card_score += 2_i32.pow(matching_numbers.len() as u32 - 1);
        }
    });

    return Ok(scratch_card_score.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA_PART1: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    
    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("13", process(TEST_DATA_PART1)?);
        Ok(())
    }
}
