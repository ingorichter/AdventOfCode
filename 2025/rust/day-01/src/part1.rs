#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut result = 0;
    let mut dial_pos = 50;

    input.lines().for_each(|line| {
        let (turn, steps) = line.split_at(1);
        let mut steps: i32 = steps.parse().unwrap();
        steps %= 100; // wrap around for large numbers
        match turn {
            "L" => {
                if dial_pos - steps < 0 {
                    dial_pos += 100 - steps;
                } else {
                    dial_pos -= steps;
                }
            },
            "R" => {
                if dial_pos + steps > 99 {
                    dial_pos = (dial_pos + steps) - 100;
                } else {
                    dial_pos += steps;
                }
            },
            _ => println!("Invalid turn direction"),
        }
        if dial_pos == 0 {
            result += 1;
        }
    });

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA_PART1: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    const TEST_DATA_PART1_LARGE_NUMBERS: &str = "L468
R91
R152
R35
R49
";

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("3", process(TEST_DATA_PART1)?);
        Ok(())
    }

    #[test]
    fn test_process_large_numbers() -> miette::Result<()> {
        assert_eq!("3", process(TEST_DATA_PART1_LARGE_NUMBERS)?);
        Ok(())
    }
}
