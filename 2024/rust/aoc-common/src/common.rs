use std::collections::HashSet;

use crate::{grid::Grid, point::Point2D};

use itertools::Itertools;

#[tracing::instrument]
pub fn make_grid(input: &str) -> Grid<char> {
    let mut grid = Grid::new(vec![]);
    for line in input.lines() {
        grid.push(line.chars().collect());
    }
    grid
}

// TODO(Ingo) make this more generic and let the caller provide a transformation function
#[tracing::instrument]
pub fn make_grid_u8(input: &str) -> Grid<u8> {
    let mut grid = Grid::new(vec![]);
    for line in input.lines() {
        grid.push(line.chars().map(|c| c as u8 - b'0').collect());
    }
    grid
}

#[tracing::instrument]
pub fn make_grid_i8(input: &str) -> Grid<i8> {
    let mut grid = Grid::new(vec![]);
    for line in input.lines() {
        grid.push(line.chars().map(|c| c.to_digit(10).unwrap() as i8).collect());
    }
    grid
}

#[tracing::instrument]
pub fn calc_shortest_path(grid: &Grid<char>, offset_factor: usize) -> i64 {
    let empty_lines = grid.iter_rows().enumerate().filter(|(_, row)| row.iter().all(|&c| c == '.')).map(|(i, _)| i).collect::<Vec<usize>>();

    let empty_columns = grid.iter_columns().enumerate().filter(|(_, column)| column.iter().all(|&c| *c == '.')).map(|(i, _)| i).collect::<Vec<usize>>();

    let mut galaxies = Vec::new();
    grid.iter_rows().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().filter(|(_, &c)| c == '#').for_each(|(x, _)| {
            galaxies.push(Point2D::new(x as i32, y as i32));
        });
    });

    let mut unique_galaxy_pairs = HashSet::new();
    for combination in galaxies.iter().combinations(2) {
        let temp_point1 = combination[0];
        let temp_point2 = combination[1];

        let empty_colums_before_point1 = empty_columns.iter().filter(|col| {
            (**col as i32) < temp_point1.x
        }).collect::<Vec<&usize>>();

        let empty_colums_before_point2 = empty_columns.iter().filter(|col| {
            (**col as i32) < temp_point2.x
        }).collect::<Vec<&usize>>();

        let empty_lines_before_point1 = empty_lines.iter().filter(|line| {
            (**line as i32) < temp_point1.y
        }).collect::<Vec<&usize>>();
        
        let empty_lines_before_point2 = empty_lines.iter().filter(|line| {
            (**line as i32) < temp_point2.y
        }).collect::<Vec<&usize>>();
        
        let point1 = Point2D::new(
            temp_point1.x + (empty_colums_before_point1.len() * (offset_factor - 1)) as i32,
            temp_point1.y + (empty_lines_before_point1.len() * (offset_factor - 1)) as i32,
        );

        let point2 = Point2D::new(
            temp_point2.x + (empty_colums_before_point2.len() * (offset_factor - 1)) as i32,
            temp_point2.y + (empty_lines_before_point2.len() * (offset_factor - 1)) as i32 ,
        );

        unique_galaxy_pairs.insert((point1, point2));
    }

    let sum: i64 = unique_galaxy_pairs.iter().map(|(point1, point2)| {
        point1.manhattan_distance(point2) as i64
    }).sum();

    sum
}
