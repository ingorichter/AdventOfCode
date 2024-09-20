use std::{char, collections::HashSet};

use itertools::Itertools;
use crate::custom_error::AocError;
use aoc_common::{grid::Grid, point::Point2D};

fn make_grid(input: &str) -> Grid<char> {
    let mut grid = Grid::new(vec![]);
    for line in input.lines() {
        grid.push(line.chars().collect());
    }
    grid
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let mut grid: Grid<char> = make_grid(input);
    // println!("{:?}", grid);
    // TODO:
    // find all lines with all '.'
    let empty_lines = grid.iter_rows().enumerate().filter(|(_, row)| row.iter().all(|&c| c == '.')).map(|(i, _)| i).collect::<Vec<usize>>();
    // println!("{:?}", empty_lines);
    // expand grid and duplicate those lines
    for (line_offset, line) in empty_lines.into_iter().enumerate() {
        grid.insert_row(line + line_offset, vec!['.'; grid.columns()]);
    }

    // println!("{:?}", grid);

    // find all columns with all '.'
    let empty_columns = grid.iter_columns().enumerate().filter(|(_, column)| column.iter().all(|&c| *c == '.')).map(|(i, _)| i).collect::<Vec<usize>>();
    // println!("{:?}", empty_columns);
    
    // expand grid and duplicate those columns
    // since we're inserting columns, we need to offset the column indices with each insert
    for (offset, col) in empty_columns.into_iter().enumerate() {
        grid.insert_column(col + offset, vec!['.'; grid.rows()]);
    }

    // println!("{:?}", grid);

    // now find all '#' and create a set of galaxy pairs
    // I'd like to have used something more efficient here. Iter, enumerate, filter, map
    let mut galaxies = Vec::new();
    grid.iter_rows().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().filter(|(_, &c)| c == '#').for_each(|(x, _)| {
            galaxies.push(Point2D::new(x as i32, y as i32));
        });
    });

    let mut unique_galaxy_pairs = HashSet::new();
    for combination in galaxies.iter().combinations(2) {
        let point1 = combination[0];
        let point2 = combination[1];
        unique_galaxy_pairs.insert((point1, point2));
    }

    // for pair in unique_galaxy_pairs.iter() {
    //     println!("{:?}", pair);
    // }

    // // Convert HashSet to Vec for sorting
    // let mut sorted_pairs: Vec<_> = unique_galaxy_pairs.into_iter().collect();

    // // Sort the pairs
    // sorted_pairs.sort_by(|a, b| a.cmp(b));

    // // Print the sorted pairs
    // for pair in &sorted_pairs {
    //     println!("{:?}", pair);
    // }

    // determine the distance of all galaxy pairs (manhattan distance)
    // sum all distances
    let sum: i32 = unique_galaxy_pairs.iter().map(|(point1, point2)| {
        point1.manhattan_distance(point2)
    }).sum();
    // return the sum
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
