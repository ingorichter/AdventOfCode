use aoc_common::common::make_grid_i8;
use aoc_common::grid::{is_safe, Grid};
use aoc_common::point::{Point2D, EAST, NORTH, SOUTH, WEST};
use std::collections::HashSet;

type TopoMap = Grid<i8>;

#[tracing::instrument]
fn possible_number_of_mountain_tops(grid: &mut TopoMap, current_pos: &Point2D) -> usize {
    fn dfs(
        grid: &mut TopoMap,
        current_pos: &Point2D,
        next_value: i8,
        visited: &mut HashSet<Point2D>,
        reachable_mountain_tops: &mut HashSet<Point2D>,
    ) {
        // base cases
        if !is_safe(current_pos, grid) {
            return;
        }

        if visited.contains(current_pos) {
            return;
        }

        let current_value = *grid.get(current_pos).unwrap();

        // next_value was already incremented and must be the value of the current_value
        if current_value != next_value {
            return;
        }

        // println!("current_value: {:?} at {:?}", current_value, current_pos);
        visited.insert(*current_pos);
        if current_value == 9 {
            reachable_mountain_tops.insert(*current_pos);
            return;
        }

        let next_value = current_value + 1;

        dfs(
            grid,
            &(*current_pos + NORTH),
            next_value,
            visited,
            reachable_mountain_tops,
        );
        dfs(
            grid,
            &(*current_pos + EAST),
            next_value,
            visited,
            reachable_mountain_tops,
        );
        dfs(
            grid,
            &(*current_pos + SOUTH),
            next_value,
            visited,
            reachable_mountain_tops,
        );
        dfs(
            grid,
            &(*current_pos + WEST),
            next_value,
            visited,
            reachable_mountain_tops,
        );
    }

    let mut reachable_mountain_tops: HashSet<Point2D> = HashSet::new();
    let mut visited: HashSet<Point2D> = HashSet::new();
    dfs(
        grid,
        current_pos,
        0,
        &mut visited,
        &mut reachable_mountain_tops,
    );

    return reachable_mountain_tops.len();
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut grid = make_grid_i8(input);

    // dbg!(grid);

    let trail_heads = grid
        .enumerate()
        .filter(|(x, y)| grid.get(&Point2D::new(*x as i32, *y as i32)) == Some(&0))
        .collect::<Vec<_>>();

    let total_paths: u16 = trail_heads
        .iter()
        .map(|(x, y)| {
            let trailhead_pos = Point2D::new(*x as i32, *y as i32);
            possible_number_of_mountain_tops(&mut grid, &trailhead_pos) as u16
        })
        .sum();

    // println!("Possible paths {:?}", total_paths);

    Ok(total_paths.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // #[ignore = "not implemented"]
    fn test_process() -> miette::Result<()> {
        let input = "0123
1234
8765
9876";
        assert_eq!("1", process(input)?);
        Ok(())
    }

    #[test]
    // #[ignore = "not implemented"]
    fn test_process_bigger() -> miette::Result<()> {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!("36", process(input)?);
        Ok(())
    }
}
