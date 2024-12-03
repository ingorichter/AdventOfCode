fn check_report_again(report: &Vec<i32>) -> bool {
    for i in 0..report.len() {
        let mut reduced = report.to_owned();
        reduced.remove(i); // remove one level

        let diffs: Vec<i32> = reduced.windows(2).map(|w| {
            w[1] - w[0]
        }).collect();

        let is_safe = diffs.iter().all(|&diff| {
            (diff == 0 || diff.signum() == diffs[0].signum()) && pred_inrange(diff)
        });

        if is_safe {
            return true
        }
    }

    false
}

fn pred_inrange(a: i32) -> bool {
    (1..=3).contains(&(a).abs())
}

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

        let all_the_same_sign = diffs.iter().all(|&diff| {
            diff == 0 || diff.signum() == diffs[0].signum()
        });
    
        match all_the_same_sign {
            true => {
                let all_in_range = diffs.iter().all(|&diff| pred_inrange(diff));
                match all_in_range {
                    true => {
                        safe_reports += 1;
                    }
                    false => {
                        let is_safe = check_report_again(report);
                        if is_safe {
                            safe_reports += 1;
                        }
                    }
                }
            }
            false => {
                let is_safe = check_report_again(report);
                if is_safe {
                    safe_reports += 1;
                }
            }
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
        assert_eq!("4", process(input)?);
        Ok(())
    }
}
