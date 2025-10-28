use aoc_common::common::make_grid_u8;
// use aoc_common::grid::Grid;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let grid = make_grid_u8(input);

    let segments = grid.find_segments();
    let sum = segments.iter().map(|s| s.perimeter * s.cells.len()).sum::<usize>();

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_grid() -> miette::Result<()> {
        let input = "AAAA
BBCD
BBCC
EEEC";
        assert_eq!("140", process(input)?);
        Ok(())
    }

    #[test]
    fn test_medium_grid() -> miette::Result<()> {
        let input = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        assert_eq!("772", process(input)?);
        Ok(())
    }

    #[test]
    fn test_bigger_grid() -> miette::Result<()> {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!("1930", process(input)?);
        Ok(())
    }
}
