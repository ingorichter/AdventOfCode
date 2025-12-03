use std::collections::{HashMap, HashSet};
use std::fmt;

use super::point::{Point2D, NORTH, SOUTH, WEST, EAST};

pub struct GridEnumerator {
    width: usize,
    height: usize,
    current_x: usize,
    current_y: usize,
}

impl Iterator for GridEnumerator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_y >= self.height {
            return None;
        }

        let result = (self.current_x, self.current_y);

        self.current_x += 1;
        if self.current_x >= self.width {
            self.current_x = 0;
            self.current_y += 1;
        }

        Some(result)
    }
}

pub struct Grid<T> {
    grid: Vec<Vec<T>>,
}

// Implement Debug for Grid<char>
impl<T: fmt::Debug> fmt::Debug for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Grid:")?;
        for row in &self.grid {
            writeln!(f, "{:?}", row)?;
        }
        Ok(())
    }
}

impl<T> Grid<T> {
    pub fn new(grid: Vec<Vec<T>>) -> Self {
        Self { grid }
    }

    pub fn rows(&self) -> usize {
        self.grid.len()
    }

    pub fn columns(&self) -> usize {
        self.grid[0].len()
    }

    pub fn get(&self, point: &Point2D) -> Option<&T> {
        // 1. Avoid double bounds checking by doing it once
        let y = point.y as usize;
        let x = point.x as usize;

        // 2. Use direct array bounds checking instead of a separate function call
        self.grid.get(y)?.get(x)
    }

    pub fn set(&mut self, point: &Point2D, value: T) {
        if !self.is_safe(point) {
            return;
        }
        self.grid[point.y as usize][point.x as usize] = value;
    }

    pub fn push(&mut self, row: Vec<T>) {
        self.grid.push(row);
    }

    pub fn iter_rows(&self) -> impl Iterator<Item = &Vec<T>> {
        self.grid.iter()
    }

    pub fn iter_columns(&self) -> impl Iterator<Item = Vec<&T>> {
        // The 'move' keyword is needed here because:
        // 1. For 'x': We need to move ownership of 'x' into the closure since it will be used across iterations
        // 2. For 'y': Similarly, we need to move 'y' since the closure needs to own the value to safely reference it
        // Without 'move', the closures would try to borrow 'x' and 'y' which could be invalid after their scope ends
        (0..self.columns()).map(move |x| (0..self.rows()).map(move |y| &self.grid[y][x]).collect())
    }

    pub fn insert_row(&mut self, index: usize, row: Vec<T>) {
        self.grid.insert(index, row);
    }

    // pub fn insert_column(&mut self, index: usize, column: Vec<T>) where T: Clone {
    //     let new_column = column.clone();
    //     (0..self.rows()).for_each(|y| self.grid[y].insert(index, new_column[y]));
    // }

    pub fn insert_column(&mut self, index: usize, column: Vec<T>)
    where
        T: Clone,
    {
        let column_iter = column.iter().cloned();
        (0..self.rows())
            .zip(column_iter)
            .for_each(|(y, value)| self.grid[y].insert(index, value));
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.grid.iter().flat_map(|row| row.iter())
    }

    // I need an iterator that yields the index of the row and the index of the column

    pub fn enumerate(&self) -> GridEnumerator {
        GridEnumerator {
            width: self.columns(),
            height: self.rows(),
            current_x: 0,
            current_y: 0,
        }
    }
    // pub fn enumerate(&self) -> impl Iterator<Item = (usize, usize)> {
    //     (0..self.rows()).flat_map(|y| (0..self.columns()).map(move |x| (x, y)))
    // }

    /// Checks if a given point is within the bounds of the grid.
    ///
    /// # Arguments
    ///
    /// * `point` - A reference to a `Point2D` representing the coordinates to check.
    /// * `grid` - A reference to the `Grid` containing the elements.
    ///
    /// # Returns
    ///
    /// * `true` if the point is within the grid bounds, `false` otherwise.
    pub fn is_safe(&self, point: &Point2D) -> bool {
        point.x >= 0
            && point.x < self.columns() as i32
            && point.y >= 0
            && point.y < self.rows() as i32

        // is_safe(point, self)
    }
}


#[derive(Debug, Clone)]
pub struct Segment {
    pub id: usize,
    pub cells: HashSet<Point2D>,
    pub neighbor_count: usize,
    pub neighbors: HashSet<usize>, // IDs of neighboring segments
    pub perimeter: usize, // Total perimeter length of the segment (individual edges)
    pub sides: usize, // Number of continuous sides (straight edges count as 1)
}

impl<T: PartialEq + Clone> Grid<T> {
    /// Finds all segments in the grid and counts their neighbors.
    /// A segment is a continuous area of cells with the same value.
    /// Segments are neighbors if they share an edge (4-directional).
    pub fn find_segments(&self) -> Vec<Segment> {
        let mut visited = vec![vec![false; self.columns()]; self.rows()];
        let mut segments = Vec::new();
        let mut segment_id = 0;

        // Find all segments using flood fill
        for (x, y) in self.enumerate() {
            if !visited[y][x] {
                let point = Point2D::new(x as i32, y as i32);
                if let Some(value) = self.get(&point) {
                    let mut cells = HashSet::new();
                    self.flood_fill(
                        &point,
                        value,
                        &mut visited,
                        &mut cells,
                    );

                    let perimeter = self.calculate_perimeter(&cells);
                    let sides = self.calculate_sides(&cells);

                    segments.push(Segment {
                        id: segment_id,
                        cells,
                        neighbor_count: 0,
                        neighbors: HashSet::new(),
                        perimeter,
                        sides,
                    });
                    segment_id += 1;
                }
            }
        }

        // Build a map from cell position to segment ID
        let mut cell_to_segment: HashMap<Point2D, usize> = HashMap::new();
        for segment in &segments {
            for cell in &segment.cells {
                cell_to_segment.insert(*cell, segment.id);
            }
        }

        // Find neighbors for each segment
        for segment in &mut segments {
            let mut neighbor_ids = HashSet::new();

            for cell in &segment.cells {
                // Check all 4 directions
                for direction in &[
                    NORTH,  // Up
                    SOUTH,   // Down
                    WEST,  // Left
                    EAST,   // Right
                ] {
                    let neighbor_pos = *cell + *direction;

                    if let Some(&neighbor_seg_id) = cell_to_segment.get(&neighbor_pos)
                        && neighbor_seg_id != segment.id {
                            neighbor_ids.insert(neighbor_seg_id);
                        }
                }
            }

            segment.neighbors = neighbor_ids.clone();
            segment.neighbor_count = neighbor_ids.len();
        }

        segments
    }

    /// Internal flood fill implementation using recursion.
    fn flood_fill(
        &self,
        point: &Point2D,
        target_value: &T,
        visited: &mut Vec<Vec<bool>>,
        cells: &mut HashSet<Point2D>,
    ) {
        // Check bounds
        if !self.is_safe(point) {
            return;
        }

        let x = point.x as usize;
        let y = point.y as usize;

        // Check if already visited
        if visited[y][x] {
            return;
        }

        // Check if value matches
        if let Some(value) = self.get(point) {
            if value != target_value {
                return;
            }
        } else {
            return;
        }

        // Mark as visited and add to segment
        visited[y][x] = true;
        cells.insert(*point);

        // Recursively check all 4 directions
        self.flood_fill(&(*point + NORTH), target_value, visited, cells); // Up
        self.flood_fill(&(*point + SOUTH), target_value, visited, cells);  // Down
        self.flood_fill(&(*point + WEST), target_value, visited, cells); // Left
        self.flood_fill(&(*point + EAST), target_value, visited, cells);  // Right
    }

    /// Iterative version of flood fill to avoid stack overflow on large segments.
    fn flood_fill_iterative(
        &self,
        start: &Point2D,
        target_value: &T,
        visited: &mut [Vec<bool>],
        cells: &mut HashSet<Point2D>,
    ) {
        let mut stack = vec![*start];

        while let Some(point) = stack.pop() {
            if !self.is_safe(&point) {
                continue;
            }

            let x = point.x as usize;
            let y = point.y as usize;

            if visited[y][x] {
                continue;
            }

            if let Some(value) = self.get(&point) {
                if value != target_value {
                    continue;
                }
            } else {
                continue;
            }

            visited[y][x] = true;
            cells.insert(point);

            // Add neighbors to stack
            stack.push(point + NORTH);  // Up
            stack.push(point + SOUTH);   // Down
            stack.push(point + WEST);  // Left
            stack.push(point + EAST);   // Right
        }
    }

    /// Calculates the perimeter of a segment.
    /// The perimeter is the number of edges that border a different segment or the grid boundary.
    fn calculate_perimeter(&self, cells: &HashSet<Point2D>) -> usize {
        let mut perimeter = 0;

        for cell in cells {
            // Check all 4 directions
            for direction in &[
                NORTH,  // Up
                SOUTH,   // Down
                WEST,  // Left
                EAST,   // Right
            ] {
                let neighbor = *cell + *direction;

                // If the neighbor is not in this segment, it contributes to the perimeter
                if !cells.contains(&neighbor) {
                    perimeter += 1;
                }
            }
        }

        perimeter
    }

    /// Calculates the number of continuous sides of a segment.
    /// A side is a continuous straight line of edges along the perimeter.
    /// This counts both outer and inner edges (holes within the segment).
    fn calculate_sides(&self, cells: &HashSet<Point2D>) -> usize {
        let mut sides = 0;

        // We'll track horizontal and vertical edges separately
        // For horizontal edges: store them as (y, x_start, x_end, is_top)
        // For vertical edges: store them as (x, y_start, y_end, is_left)

        let mut horizontal_edges: HashMap<(i32, bool), Vec<(i32, i32)>> = HashMap::new();
        let mut vertical_edges: HashMap<(i32, bool), Vec<(i32, i32)>> = HashMap::new();

        // Collect all perimeter edges
        for cell in cells {
            // Check top edge
            if !cells.contains(&(*cell + Point2D::new(0, -1))) {
                horizontal_edges
                    .entry((cell.y, true))
                    .or_default()
                    .push((cell.x, cell.x + 1));
            }

            // Check bottom edge
            if !cells.contains(&(*cell + Point2D::new(0, 1))) {
                horizontal_edges
                    .entry((cell.y + 1, false))
                    .or_default()
                    .push((cell.x, cell.x + 1));
            }

            // Check left edge
            if !cells.contains(&(*cell + Point2D::new(-1, 0))) {
                vertical_edges
                    .entry((cell.x, true))
                    .or_default()
                    .push((cell.y, cell.y + 1));
            }

            // Check right edge
            if !cells.contains(&(*cell + Point2D::new(1, 0))) {
                vertical_edges
                    .entry((cell.x + 1, false))
                    .or_default()
                    .push((cell.y, cell.y + 1));
            }
        }

        // Merge horizontal edges into continuous sides
        for (_, mut segments) in horizontal_edges {
            segments.sort();
            let merged = Self::merge_segments(segments);
            sides += merged.len();
        }

        // Merge vertical edges into continuous sides
        for (_, mut segments) in vertical_edges {
            segments.sort();
            let merged = Self::merge_segments(segments);
            sides += merged.len();
        }

        sides
    }

    /// Merges overlapping or adjacent segments into continuous segments.
    fn merge_segments(segments: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
        if segments.is_empty() {
            return vec![];
        }

        let mut merged = vec![];
        let mut current_start = segments[0].0;
        let mut current_end = segments[0].1;

        for &(start, end) in segments.iter().skip(1) {
            if start <= current_end {
                // Overlapping or adjacent, extend the current segment
                current_end = current_end.max(end);
            } else {
                // Gap found, save current segment and start a new one
                merged.push((current_start, current_end));
                current_start = start;
                current_end = end;
            }
        }

        // Don't forget the last segment
        merged.push((current_start, current_end));
        merged
    }
}

// Example usage:
//
// let grid = Grid::new(vec![
//     vec!['A', 'A', 'A', 'A'],
//     vec!['B', 'B', 'C', 'D'],
//     vec!['B', 'B', 'C', 'C'],
//     vec!['E', 'E', 'E', 'C'],
// ]);
//
// let segments = grid.find_segments();
// for segment in segments {
//     println!("Segment {}: {} cells, {} neighbors",
//              segment.id, segment.cells.len(), segment.neighbor_count);
// }