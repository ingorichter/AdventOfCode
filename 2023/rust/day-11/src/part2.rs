use std::char;

use crate::custom_error::AocError;
use aoc_common::grid::Grid;
use crate::common::make_grid;
use crate::common::calc_shortest_path;

pub fn calc(input: &str, factor: usize) -> miette::Result<String, AocError> {
    let grid: Grid<char> = make_grid(input);
    let sum: i64 = calc_shortest_path(&grid, factor);
    Ok(sum.to_string())
}

#[tracing::instrument]
pub fn process(
    input: &str
) -> miette::Result<String, AocError> {
    calc(input, 1_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA_PART1: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_calc_2() -> miette::Result<()> {
        assert_eq!("374", calc(TEST_DATA_PART1, 2)?);
        Ok(())
    }

    #[test]
    fn test_calc_10() -> miette::Result<()> {
        assert_eq!("1030", calc(TEST_DATA_PART1, 10)?);
        Ok(())
    }

    #[test]
    fn test_calc_100() -> miette::Result<()> {
        assert_eq!("8410", calc(TEST_DATA_PART1, 100)?);
        Ok(())
    }
}
