use aoc_common::common::make_grid_i8;
use aoc_common::grid::{is_safe, Grid};
use aoc_common::point::{Point2D, EAST, NORTH, SOUTH, WEST};

type TopoMap = Grid<i8>;
const VISITED: i8 = -1;

#[tracing::instrument]
fn possible_number_of_trails_to_mountain_tops(grid: &mut TopoMap, current_pos: &Point2D) -> u16 {
    fn dfs(
        grid: &mut TopoMap,
        current_pos: &Point2D,
        next_value: i8,
        path_count: &mut u16,
    ) {
        // base cases
        if !is_safe(current_pos, grid) {
            return;
        }

        let current_value = *grid.get(current_pos).unwrap();

        // next_value was already incremented and must be the value of the current_value
        if current_value != next_value {
            return;
        }

        // println!("current_value: {:?} at {:?}", current_value, current_pos);
        if current_value == 9 {
            *path_count += 1;
            return;
        }

        // temp set cell to visited
        grid.set(current_pos, VISITED);
        let next_value = current_value + 1;

        dfs(
            grid,
            &(*current_pos + NORTH),
            next_value,
            path_count,
        );
        dfs(
            grid,
            &(*current_pos + EAST),
            next_value,
            path_count,
        );
        dfs(
            grid,
            &(*current_pos + SOUTH),
            next_value,
            path_count,
        );
        dfs(
            grid,
            &(*current_pos + WEST),
            next_value,
            path_count,
        );

        grid.set(current_pos, current_value);
    }

    let mut total_path_count:u16 = 0;
    dfs(
        grid,
        current_pos,
        0,
        &mut total_path_count
    );

    return total_path_count;
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
            possible_number_of_trails_to_mountain_tops(&mut grid, &trailhead_pos) as u16
        })
        .sum();

    // println!("Possible paths {:?}", total_paths);

    Ok(total_paths.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!("81", process(input)?);
        Ok(())
    }
}
