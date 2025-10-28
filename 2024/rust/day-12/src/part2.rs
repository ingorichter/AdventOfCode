use aoc_common::common::make_grid_u8;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let grid = make_grid_u8(input);

    let segments = grid.find_segments();
    let sum = segments.iter().map(|s| s.sides * s.cells.len()).sum::<usize>();

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
        assert_eq!("80", process(input)?);
        Ok(())
    }

    #[test]
    fn test_medium_grid() -> miette::Result<()> {
        let input = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
        assert_eq!("236", process(input)?);
        Ok(())
    }

    #[test]
    fn test_bigger_grid() -> miette::Result<()> {
        let input = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
        assert_eq!("368", process(input)?);
        Ok(())
    }
}
