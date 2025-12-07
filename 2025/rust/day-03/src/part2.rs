#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let battery_banks: Vec<Vec<i8>> = parse_input(input);
    // info!("Parsed battery bank: {:?}", battery_banks);

    let result = battery_banks.iter().map(|bank| {
        find_maximum_joltage_sum(bank, 12)
    }).map(|vec| vec_to_number(&vec)).sum::<i64>();

    Ok(result.to_string())
}

fn vec_to_number(vec: &[i8]) -> i64 {
    vec.iter().fold(0, |acc, &digit| acc * 10 + digit as i64)
}

fn find_maximum_joltage_sum(battery_bank: &[i8], k: usize) -> Vec<i8> {
    let mut result: Vec<i8> = Vec::new();

    // info!("Processing battery bank: {:?}", battery_bank);

    let digits_in_bank = battery_bank.len();
    let mut start_index = 0;

    for i in 0..k {
        let digits_needed = k - i - 1;
        let search_end = digits_in_bank - digits_needed;

        let max_digit_in_range = *battery_bank[start_index..search_end].iter().max().unwrap();

        for (j, value) in battery_bank.iter().enumerate().take(search_end).skip(start_index) {
        // for j in start_index..search_end {
            if *value == max_digit_in_range {
                result.push(*value);
                start_index = j + 1;
                break;
            }
        }
    }

    // info!("Highest joltage : {:?}", result);

    result
}

fn parse_input(input: &str) -> Vec<Vec<i8>> {
    input.lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as i8).collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!("3121910778619", process(input)?);
        Ok(())
    }
}
