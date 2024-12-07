use aoc_common::{common::make_grid, grid::Grid};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let grid: Grid<char> = make_grid(input);
    dbg!(&grid);
    Ok("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!("41", process(input)?);
        Ok(())
    }
}
