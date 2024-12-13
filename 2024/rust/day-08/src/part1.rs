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
    // dbg!(&antenna_group_map);
    let antenna_pairs: Vec<(Point2D, Point2D)> = antenna_group_map
        .into_iter()
        .flat_map(|(_antenna, positions)| {
            positions
                .into_iter()
                .tuple_combinations()
        })
        .collect();
    // dbg!(&antenna_pairs);
    let antinodes: Vec<_> = antenna_pairs
        .into_iter()
        .map(|(pos1, pos2)| {
            let distance = pos1 - pos2;
            let (a1, a2) = make_antinodes(pos1, pos2, distance);
            // println!(
            //     "Antinodes for ({:?} and {:?}) with distance {:?} are ({:?} {:?})",
            //     pos1, pos2, distance, &a1, &a2
            // );
            (a1, a2)
        })
        .collect();
    // dbg!(&antinodes.len());
    let (first_points, second_points): (Vec<Point2D>, Vec<Point2D>) = antinodes.into_iter().unzip();
    // dbg!(&first_points, &second_points);
    let mut first_points = first_points
        .into_iter()
        .filter(|p| is_safe(p, &grid))
        .collect::<HashSet<Point2D>>();
    let second_points = second_points
        .into_iter()
        .filter(|p| is_safe(p, &grid))
        .collect::<HashSet<Point2D>>();

    first_points.extend(second_points.iter());

    let antinode_count = first_points.len();

    Ok(antinode_count.to_string())
}

fn make_antinodes(p0: Point2D, p1: Point2D, distance: Point2D) -> (Point2D, Point2D) {
    // dbg!(&p0, &p1, &distance);
    if p0.y > p1.y {
        (p0 - distance, p1 + distance)
    } else {
        (p0 + distance, p1 - distance)
    }
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
        assert_eq!("14", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_simple() -> miette::Result<()> {
        let input = "............
........0...
.....0......
............
............
............
............
............
............
............
............
............";
        assert_eq!("2", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_simple_small() -> miette::Result<()> {
        let input = ".....
..A..
.....
.A...
.....
.....";
        assert_eq!("1", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_simple_2() -> miette::Result<()> {
        let input = "............
............
............
............
.....A......
............
............
............
.........A..
............
............
............";
        assert_eq!("1", process(input)?);
        Ok(())
    }
}
