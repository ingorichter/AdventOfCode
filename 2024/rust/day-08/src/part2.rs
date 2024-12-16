use std::collections::HashSet;

use aoc_common::common::make_grid;
use aoc_common::grid::is_safe;
use aoc_common::point::Point2D;
use itertools::Itertools;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let grid = make_grid(input);
    // dbg!(&grid);
    // create a list with all antennas and their position
    let antennas: Vec<(char, Point2D)> = grid
        .enumerate()
        .filter_map(|(x, y)| {
            let pos = Point2D::new(x as i32, y as i32);
            grid.get(&pos)
                .and_then(|&ch| if ch != '.' { Some((ch, pos)) } else { None })
        })
        .collect();

    // dbg!(&antennas);
    // build a list with all antennas
    // build pairs of antennas
    // determine their distance and create two antinodes with the same distance
    // filter out all antinodes that are outside the grid
    let antenna_group_map = antennas.into_iter().into_group_map();
    let antenna_pairs: Vec<(Point2D, Point2D)> = antenna_group_map
        .into_iter()
        .flat_map(|(_antenna, positions)| positions.into_iter().tuple_combinations())
        .collect();
    let mut antinodes_set1: HashSet<Point2D> = HashSet::new();
    antenna_pairs.iter().for_each(|(pos1, pos2)| {
        let distance = *pos1 - *pos2;
        let mut new_pos = *pos1 + distance;
        antinodes_set1.insert(*pos1);
        while is_safe(&new_pos, &grid) {
            antinodes_set1.insert(new_pos);
            new_pos += distance;
        }
    });

    let antinodes_set2: HashSet<Point2D> = antenna_pairs
        .iter()
        .flat_map(|(pos1, pos2)| {
            let distance = *pos1 - *pos2;
            let mut antinodes = vec![*pos1];
            let mut new_pos = *pos1 - distance;
            while is_safe(&new_pos, &grid) {
                antinodes.push(new_pos);
                new_pos -= distance;
            }
            antinodes
        })
        .collect();

    antinodes_set1.extend(&antinodes_set2);

    let antinode_count = antinodes_set1.len();

    Ok(antinode_count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!("34", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_simple() -> miette::Result<()> {
        let input = "............
............
............
...#........
............
.....#......
............
............
............
............
............
............";
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
