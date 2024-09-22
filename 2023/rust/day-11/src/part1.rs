use std::char;

use crate::custom_error::AocError;
use aoc_common::grid::Grid;
use crate::common::make_grid;
use crate::common::calc_shortest_path;

#[tracing::instrument]
pub fn process(
    input: &str    
) -> miette::Result<String, AocError> {
    let grid: Grid<char> = make_grid(input);
    let sum = calc_shortest_path(&grid, 1);
    Ok(sum.to_string())
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
    fn test_process() -> miette::Result<()> {
        assert_eq!("374", process(TEST_DATA_PART1)?);
        Ok(())
    }
}
