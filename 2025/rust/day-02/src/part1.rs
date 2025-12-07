use crate::input_parser::parse_input;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let ranges = match parse_input(input) {
        Ok((_remaining, ranges)) => {
            Ok(ranges)
        }
        Err(e) => {
            Err(miette::miette!("Failed to parse input: {}", e))
        }
    }.expect("TODO: panic message");

    let result: u64 = ranges.iter().map(|range| {
        let doubled_blocks = doubled_blocks_in_range_generated(range.start, range.end);

        // Here you can accumulate results as needed
        // For example, counting them or summing them up
        doubled_blocks.iter().sum::<u64>()
    }).sum();

    Ok(result.to_string())
}

fn pow10(exp: u32) -> u64 {
    10u64.pow(exp)
}

fn doubled_blocks_in_range_generated(l: u64, r: u64) -> Vec<u64> {
    let max_len = r.to_string().len();
    let mut res = Vec::new();

    // d = number of digits in X
    for d in 1..=((max_len / 2) as u32) {
        let start = pow10(d - 1);      // smallest d-digit number (no leading zeros)
        let end = pow10(d) - 1;        // largest  d-digit number

        let factor = pow10(d);         // 10^d

        for x in start..=end {
            let n = x * factor + x;    // "concatenate" x with itself

            if n > r {
                break;                 // further x will only make n bigger
            }
            if n >= l {
                res.push(n);
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";
        assert_eq!("1227775554", process(input)?);
        Ok(())
    }
}
