use std::collections::HashSet;

use aoc_common::{
    common::make_grid,
    grid::Grid,
    point::{Point2D, NORTH},
};

type LabMap = Grid<char>;

const GUARD: char = '^';
const VISITED: char = 'X';
const OBSTACLE: char = '#';
const EMPTY: char = '.';

#[tracing::instrument]
fn walk_the_grid(guard_position: &Point2D, grid: &LabMap) -> (HashSet<Point2D>, bool) {
    let mut direction = NORTH;
    let mut location = *guard_position;
    let mut visited_positions: HashSet<(Point2D, Point2D)> = HashSet::new();

    // iterate as long as the guard is inside the grid
    while grid.is_safe(&location) && !visited_positions.contains(&(location, direction)) {
        visited_positions.insert((location, direction));
        let new_position = location + direction;

        if grid
            .get(&new_position)
            .copied()
            .is_some_and(|x| x == OBSTACLE)
        {
            direction = direction.rotate_clockwise()
        } else {
            location = new_position;
        }
    }

    let v: HashSet<Point2D> = visited_positions.iter().map(|(pos, _)| *pos).collect();
    // dbg!(&v);

    return (v, grid.is_safe(&location));
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut grid = make_grid(input);
    // agent is always walking north in the beginning
    let guard_position = find_guard_position(&grid);

    let visited_positions = walk_the_grid(&guard_position, &grid);

    let positions = visited_positions
        .0
        .iter()
        .filter(|pos| {
            grid.set(pos, OBSTACLE);
            let res = walk_the_grid(&guard_position, &grid);
            grid.set(pos, EMPTY);
            res.1
        })
        .collect::<Vec<_>>();

    Ok(positions.len().to_string())
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
        .find(|(x, y)| {
            let pos = Point2D::new(*x as i32, *y as i32);
            grid.get(&pos).copied() == Some(GUARD)
        })
        .map(|(x, y)| Point2D::new(x as i32, y as i32))
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
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
