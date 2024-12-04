use regex::Regex;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    // Regex to match mul(x,y) where x and y are integers
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    // Capture all matches and extract x, y as integers
    let results: Vec<(i64, i64)> = re
        .captures_iter(input)
        .filter_map(|cap| {
            let x = cap[1].parse::<i64>().ok();
            let y = cap[2].parse::<i64>().ok();
            match (x, y) {
                (Some(x), Some(y)) => Some((x, y)),
                _ => None,
            }
        })
        .collect();

    let result = results.iter().fold(0, |acc, (x, y)| acc + x * y);

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("161", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_two_lines() -> miette::Result<()> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
        xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("322", process(input)?);
        Ok(())
    }
}
