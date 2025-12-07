use std::ops::Range;
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
        let invalid_ids = repeated_pattern_numbers_in_range(range);

        // Here you can accumulate results as needed
        // For example, counting them or summing them up
        invalid_ids.iter().sum::<u64>()
    }).sum();

    Ok(result.to_string())
}

fn repeated_pattern_numbers_in_range(range: &Range<u64>) -> Vec<u64> {
    let mut res = Vec::new();

    for n in range.start..=range.end {
        if is_repeated_pattern_number(n) {
            res.push(n);
        }
    }
    res
}

fn is_repeated_pattern_number(n: u64) -> bool {
    let s = n.to_string();
    let bytes = s.as_bytes();
    let len = bytes.len();

    // Pattern length must be < len and must divide len.
    // We only need to try up to len / 2 (at least 2 repeats).
    for pat_len in 1..=len / 2 {
        if len.is_multiple_of(pat_len) {
            let pattern = &bytes[..pat_len];
            let mut ok = true;
            let mut i = pat_len;

            while i < len {
                if &bytes[i..i + pat_len] != pattern {
                    ok = false;
                    break;
                }
                i += pat_len;
            }

            if ok {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";
        assert_eq!("4174379265", process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn test_is_repeated_pattern_number() {
        assert!(is_repeated_pattern_number(1212));
        assert!(is_repeated_pattern_number(123123));
        assert!(is_repeated_pattern_number(999999));
        assert!(!is_repeated_pattern_number(1234));
        assert!(!is_repeated_pattern_number(123321));
        assert!(!is_repeated_pattern_number(1234567));
    }
}
