use aoc_common::{
    common::make_grid,
    grid::{is_safe, Grid},
    point::{Point2D, NORTH},
};

type LabMap = Grid<char>;

const GUARD: char = '^';
const VISITED: char = 'X';
const OBSTACLE: char = '#';

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut grid = make_grid(input);
    // agent is always walking north in the beginning
    let mut guard_position = find_guard_position(&grid);
    let mut direction = NORTH;

    // iterate as long as the guard is inside the grid
    while is_safe(&guard_position, &grid) {
        let new_position = guard_position + direction;

        grid.set(&guard_position, VISITED);

        if grid
            .get(&new_position)
            .copied()
            .is_some_and(|x| x == OBSTACLE)
        {
            direction = direction.rotate_clockwise()
        } else {
            guard_position = new_position;
        }
    }

    let distinct_positions = find_distinct_positions(&grid);

    Ok(distinct_positions.len().to_string())
}

#[tracing::instrument]
fn find_distinct_positions(grid: &LabMap) -> Vec<Point2D> {
    grid.enumerate()
        .filter_map(|(x, y)| {
            let x = x as i32;
            let y = y as i32;
            let pos = Point2D::new(x, y);
            if grid.get(&pos).copied() == Some(VISITED) {
                Some(Point2D::new(x, y))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

#[tracing::instrument]
fn find_guard_position(grid: &LabMap) -> Point2D {
    grid.enumerate()
        .find_map(|(x, y)| {
            let x = x as i32;
            let y = y as i32;
            let pos = Point2D::new(x, y);
            (grid.get(&pos).copied() == Some(GUARD)).then_some(Point2D::new(x, y))
        })
        .unwrap()
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
