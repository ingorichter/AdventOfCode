use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::common::is_safe;
use crate::common::make_grid;
use crate::common::Grid;
use crate::custom_error::AocError;
use crate::common::Tile;
use crate::common::Point2D;
use crate::common::{EAST, NORTH, SOUTH, WEST};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    // mapping from tile and direction that we were moving to the new direction
    let directions: HashMap<(Tile, Point2D), Point2D> = HashMap::from([
        ((Tile::VerticalPipe, SOUTH), SOUTH),
        ((Tile::VerticalPipe, NORTH), NORTH),
        ((Tile::HorizontalPipe, EAST), EAST),
        ((Tile::HorizontalPipe, WEST), WEST),
        ((Tile::NorthEastBend, WEST), NORTH),
        ((Tile::NorthEastBend, SOUTH), EAST),
        ((Tile::NorthWestBend, SOUTH), WEST),
        ((Tile::NorthWestBend, EAST), NORTH),
        ((Tile::SouthWestBend, EAST), SOUTH),
        ((Tile::SouthWestBend, NORTH), WEST),
        ((Tile::SouthEastBend, WEST), SOUTH),
        ((Tile::SouthEastBend, NORTH), EAST),
    ]);

    let grid: Grid<Tile> = make_grid(input);

    // dbg!(&grid);

    // find start
    let mut start_point = Point2D::new(-1, -1);

    // what I really want is an iterator that returns each cell and the point it represents
    for y in 0..grid.rows() {
        for x in 0..grid.columns() {
            let point = Point2D::new(x as i32, y as i32);
            if *grid.get(&point).unwrap() == Tile::Start {
                start_point = point;
                break;
            }
        }
    }

    // dbg!(&start_point);

    let valid_neighbors = start_point
        .neighbors()
        .into_iter()
        .filter(|n| is_safe(n, &grid))
        .collect::<Vec<Point2D>>();
    // dbg!(&valid_neighbors);

    // find the first safe neighbor that we can move to
    let start_pipe_neighbor = valid_neighbors
        .iter()
        .find_or_first(|p| {
            let tile = &grid.get(p).unwrap();
            let dir = p.sub(&start_point);
            directions.contains_key(&(**tile, dir))
        })
        .unwrap();

    // dbg!(start_pipe_neighbor);

    // all the points in the pipe
    let mut whole_pipe: HashSet<Point2D> = HashSet::new();
    whole_pipe.insert(start_point);
    let mut current = *start_pipe_neighbor;
    let mut direction = start_pipe_neighbor.sub(&start_point);

    // traverse grid
    while current != start_point {
        whole_pipe.insert(current);

        let tile = &grid.get(&current).unwrap();
        // lookup the direction based on the current tile and the direction we are going
        let next_direction = directions.get(&(**tile, direction)).unwrap();
        direction = *next_direction;
        current = current.add(&direction);
    }

    return Ok((whole_pipe.len() / 2).to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA_PART1: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("8", process(TEST_DATA_PART1)?);
        Ok(())
    }
}
