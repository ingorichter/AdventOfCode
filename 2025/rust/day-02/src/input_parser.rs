use nom::Parser;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, newline, space0};
use nom::combinator::{map_res, opt};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::{separated_pair, terminated};
use std::ops::Range;

/// Step 1: Parse a single number
/// `digit1` matches one or more digits
/// `map_res` converts the string slice to u64
fn parse_number(input: &str) -> IResult<&str, u64> {
    map_res(digit1, |s: &str| s.parse::<u64>()).parse(input)
}

/// Step 2: Parse a range like "11-22"
/// `separated_pair` combines two parsers with a separator
fn parse_range(input: &str) -> IResult<&str, Range<u64>> {
    let (input, (start, end)) = separated_pair(
        parse_number,  // First number
        tag("-"),      // Separator
        parse_number,  // Second number
    ).parse(input)?;

    Ok((input, Range { start, end }))
}

/// Step 3: Parse comma-separated ranges, handling optional newlines
/// This allows for multiline input like the example
fn parse_ranges(input: &str) -> IResult<&str, Vec<Range<u64>>> {
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
pub fn parse_input(input: &str) -> IResult<&str, Vec<Range<u64>>> {
    terminated(parse_ranges, opt(newline)).parse(input)
}
