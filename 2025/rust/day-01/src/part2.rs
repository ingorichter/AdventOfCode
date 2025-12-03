const DIAL_TICKS: i32 = 100;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut result = 0;
    let mut dial_pos = 50;

    input.lines().for_each(|line| {
        let (turn, steps) = line.split_at(1);
        let steps: i32 = steps.parse().unwrap();
        match turn {
            "L" => {
                let (new_dial_pos, extra_rotations) = turn_dial(dial_pos, -steps);
                dial_pos = new_dial_pos;
                result += extra_rotations;
            },
            "R" => {
                let (new_dial_pos, extra_rotations) = turn_dial(dial_pos, steps);
                dial_pos = new_dial_pos;
                result += extra_rotations;
            },
            _ => println!("Invalid turn direction"),
        }
    });

    Ok(result.to_string())
}

fn turn_dial(dial_pos: i32, rotations: i32) -> (i32, i32) {
    let new_dial_pos = dial_pos + rotations;
    let mut extra_rotations = (new_dial_pos / DIAL_TICKS).abs();

    if dial_pos != 0 && new_dial_pos <= 0 {
        extra_rotations += 1;
    }

    (new_dial_pos.rem_euclid(DIAL_TICKS), extra_rotations)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
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

        assert_eq!("6", process(TEST_DATA_PART1)?);
        Ok(())
    }

    #[test_log::test]
    #[ignore]
    fn test_process_large_numbers() -> miette::Result<()> {
        const TEST_DATA_PART1_LARGE_NUMBERS: &str = "L50
R99
L100
R50
R50
";

        assert_eq!("3", crate::part1::process(TEST_DATA_PART1_LARGE_NUMBERS)?);
        Ok(())
    }

    #[test_log::test]
    fn test_turn_dial() {
        assert_eq!((0, 1), turn_dial(50, -50));
        assert_eq!((49, 0), turn_dial(50, -1));
        assert_eq!((0, 1), turn_dial(99, 1));
        assert_eq!((51, 0), turn_dial(50, 1));
        assert_eq!((95, 1), turn_dial(5, -10));
        assert_eq!((10, 2), turn_dial(10, 200));
    }
}
