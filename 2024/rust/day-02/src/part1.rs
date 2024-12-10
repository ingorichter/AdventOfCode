#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut reports = vec![];
    let mut safe_reports = 0;

    input.lines().for_each(|line| {
        let report: Result<Vec<i32>, _> = line.split_whitespace().map(str::parse::<i32>).collect();
        reports.push(report.unwrap());
    });

    for report in reports.iter() {
        let diffs: Vec<i32> = report.windows(2).map(|w| {
            w[1] - w[0]
        }).collect();

        let all_the_same_sign_and_inrange = diffs.iter().all(|&diff| {
            (diff == 0 || diff.signum() == diffs[0].signum()) && (1..=3).contains(&(diff).abs())
        });

        if all_the_same_sign_and_inrange {
            safe_reports += 1;
        }
    }

    Ok(safe_reports.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
