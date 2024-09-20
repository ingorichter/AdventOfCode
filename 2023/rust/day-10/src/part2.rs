use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::common::{is_safe, make_grid, Grid, Point2D, Tile, EAST, NORTH, SOUTH, WEST};
use crate::custom_error::AocError;

fn get_start_pipe(grid: &Grid<Tile>, directions: &HashMap<(Tile, Point2D), Point2D>, start: Point2D) -> Tile {
    let valid_neighbors = start
        .neighbors()
        .into_iter()
        .filter(|n| is_safe(n, grid))
        .collect::<Vec<Point2D>>();
    // dbg!(&valid_neighbors);
    // dbg!(&start);

    let start_pipe_neighbor = valid_neighbors
    .iter()
    .filter(|p| {
        let tile = &grid.get(p).unwrap();
        let dir = p.sub(&start);
        directions.contains_key(&(**tile, dir))
    }).collect::<Vec<&Point2D>>();

    let north = start_pipe_neighbor
        .iter()
        .any(|coord| coord.y < start.y);
    let south = start_pipe_neighbor
        .iter()
        .any(|coord| coord.y > start.y);
    let west = start_pipe_neighbor
        .iter()
        .any(|coord| coord.x < start.x);
    let east = start_pipe_neighbor
        .iter()
        .any(|coord| coord.x > start.x);

    match (north, west, south, east) {
        (true, true, _, _) => Tile::NorthWestBend,
        (true, _, true, _) => Tile::VerticalPipe,
        (true, _, _, true) => Tile::NorthEastBend,
        (_, true, true, _) => Tile::SouthWestBend,
        (_, _, true, true) => Tile::SouthEastBend,
        (_, true, _, true) => Tile::HorizontalPipe,
        _ => panic!("No valid tile to replace Start with was found"),
    }
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
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

    let grid = make_grid(input);

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
    let start_pipe = get_start_pipe(&grid, &directions, start_point);

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

    // on/off counting prep
    let mut cleaned_grid = Grid::new(vec![vec![Tile::Ground; grid.columns()]; grid.rows()]);
    // copy the pipe into the cleaned grid
    for p in whole_pipe {
        let tile = &grid.get(&p).unwrap();

        if *tile == &Tile::Start {
            cleaned_grid.set(&p, start_pipe);
            continue;
        }
        cleaned_grid.set(&p, **tile);
    }
    // dbg!(&cleaned_grid);

    let mut inside = false;
    let mut inside_count = 0;

    for y in 0..cleaned_grid.rows() {
        for x in 0..cleaned_grid.columns() {
            let point = Point2D::new(x as i32, y as i32);
            let tile = *cleaned_grid.get(&point).unwrap();

            match tile {
                Tile::VerticalPipe | Tile::NorthEastBend | Tile::NorthWestBend => {
                    inside = !inside;
                },
                Tile::Ground => {
                    inside_count += inside as i32;
                },
                _ => {
                    // inside = false;
                },
            }
            // dbg!(inside_count);
            // dbg!(inside);
            // dbg!(&point);
        }
        inside = false;
    }
    
    return Ok(inside_count.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA_PART2: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    const TEST_DATA_PART2_2: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

    const TEST_DATA_PART2_3: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("4", process(TEST_DATA_PART2)?);
        Ok(())
    }

    #[test]
    fn test_process2() -> miette::Result<()> {
        assert_eq!("8", process(TEST_DATA_PART2_2)?);
        Ok(())
    }

    #[test]
    fn test_process3() -> miette::Result<()> {
        assert_eq!("10", process(TEST_DATA_PART2_3)?);
        Ok(())
    }
}
