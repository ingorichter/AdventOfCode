#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let battery_banks: Vec<Vec<i16>> = parse_input(input);
    // info!("Parsed battery bank: {:?}", battery_banks);

    let result = battery_banks.iter().map(find_maximum_joltage_sum).sum::<i16>();

    Ok(result.to_string())
}

fn find_maximum_joltage_sum(battery_bank: &Vec<i16>) -> i16 {
    let mut highest_joltage = 0;

    // info!("Processing battery bank: {:?}", battery_bank);

    for outer in 0..battery_bank.len() - 1 {
        for inner in outer + 1..battery_bank.len() {
            let current_joltage = battery_bank[outer] * 10 + battery_bank[inner];
            if current_joltage > highest_joltage {
                highest_joltage = current_joltage;
            }
        }
    }

    // info!("Highest joltage sum: {}", highest_joltage);
    highest_joltage
}

fn parse_input(input: &str) -> Vec<Vec<i16>> {
    input.lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as i16).collect()).collect()
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
        assert_eq!("357", process(input)?);
        Ok(())
    }
}
