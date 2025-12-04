use nom::bytes::complete::tag;
use nom::character::complete::{digit1, newline, space0};
use nom::combinator::{map_res, opt};
use nom::{IResult, Parser};
use nom::multi::separated_list1;
use nom::sequence::{separated_pair, terminated};

#[derive(Debug, PartialEq)]
struct Range {
    start: u64,
    end: u64,
}

/// Step 1: Parse a single number
/// `digit1` matches one or more digits
/// `map_res` converts the string slice to u64
fn parse_number(input: &str) -> IResult<&str, u64> {
    map_res(digit1, |s: &str| s.parse::<u64>()).parse(input)
}

/// Step 2: Parse a range like "11-22"
/// `separated_pair` combines two parsers with a separator
fn parse_range(input: &str) -> IResult<&str, Range> {
    let (input, (start, end)) = separated_pair(
        parse_number,  // First number
        tag("-"),      // Separator
        parse_number,  // Second number
    ).parse(input)?;

    Ok((input, Range { start, end }))
}

/// Step 3: Parse comma-separated ranges, handling optional newlines
/// This allows for multiline input like the example
fn parse_ranges(input: &str) -> IResult<&str, Vec<Range>> {
    // We need to handle commas that might be followed by newlines
    // separated_list1 requires at least one item
    separated_list1(
        // Separator: comma, optional spaces, optional newline, optional spaces
        |i| {
            let (i, _) = tag(",").parse(i)?;
            let (i, _) = space0.parse(i)?;
            let (i, _) = opt(newline).parse(i)?;
            let (i, _) = space0.parse(i)?;
            Ok((i, ()))
        },
        parse_range,
    ).parse(input)
}

/// Step 4: Complete parser that handles trailing newlines
fn parse_input(input: &str) -> IResult<&str, Vec<Range>> {
    terminated(parse_ranges, opt(newline)).parse(input)
}

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
        let l = range.start;
        let r = range.end;

        let doubled_blocks = doubled_blocks_in_range_generated(l, r);
        // dbg!(&doubled_blocks);

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
