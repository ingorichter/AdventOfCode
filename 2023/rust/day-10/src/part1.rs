use std::{
    collections::{HashMap, HashSet},
    ops,
};

use itertools::Itertools;

use crate::custom_error::AocError;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Tile {
    VerticalPipe,
    HorizontalPipe,
    NorthEastBend,
    NorthWestBend,
    SouthWestBend,
    SouthEastBend,
    Ground,
    Start,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point2D {
    x: i32,
    y: i32,
}

const NORTH: Point2D = Point2D { x: 0, y: -1 };
const SOUTH: Point2D = Point2D { x: 0, y: 1 };
const WEST: Point2D = Point2D { x: -1, y: 0 };
const EAST: Point2D = Point2D { x: 1, y: 0 };

impl Point2D {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn add(&self, other: &Point2D) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn sub(&self, other: &Point2D) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    fn neighbors(&self) -> Vec<Point2D> {
        vec![
            self.add(&NORTH),
            self.add(&SOUTH),
            self.add(&WEST),
            self.add(&EAST),
        ]
    }
}

impl ops::Add<Point2D> for Point2D {
    type Output = Point2D;

    fn add(self, other: Point2D) -> Point2D {
        Point2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::Sub<Point2D> for Point2D {
    type Output = Point2D;

    fn sub(self, other: Point2D) -> Point2D {
        Point2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

fn is_safe<T>(point: &Point2D, grid: &Grid<T>) -> bool {
    if point.x < 0 || point.x >= grid.columns() as i32 {
        return false;
    }
    if point.y < 0 || point.y >= grid.rows() as i32 {
        return false;
    }
    true
}

#[derive(Debug)]
struct Grid<T> {
    grid: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    fn new(grid: Vec<Vec<T>>) -> Self {
        Self { grid }
    }

    fn rows(&self) -> usize {
        self.grid.len()
    }

    fn columns(&self) -> usize {
        self.grid[0].len()
    }

    fn get(&self, point: &Point2D) -> Option<&T> {
        if !is_safe(point, self) {
            return None;
        }
        Some(&self.grid[point.y as usize][point.x as usize])
    }

    fn set(&mut self, point: &Point2D, value: T) {
        if !is_safe(point, self) {
            return;
        }
        self.grid[point.y as usize][point.x as usize] = value;
    }

    fn push(&mut self, row: Vec<T>) {
        self.grid.push(row);
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut grid: Grid<Tile> = Grid::new(vec![]);

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

    input.lines().for_each(|line| {
        // grid.push(line.chars().collect());
        let mut row: Vec<Tile> = vec![];
        line.chars().for_each(|c| match c {
            '|' => row.push(Tile::VerticalPipe),
            '-' => row.push(Tile::HorizontalPipe),
            'L' => row.push(Tile::NorthEastBend),
            'F' => row.push(Tile::SouthEastBend),
            '7' => row.push(Tile::SouthWestBend),
            'J' => row.push(Tile::NorthWestBend),
            '.' => row.push(Tile::Ground),
            'S' => row.push(Tile::Start),
            _ => (),
        });
        grid.push(row);
    });

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
