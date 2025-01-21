use std::fmt;

use super::point::Point2D;

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

    pub fn iter_rows(&self) -> std::slice::Iter<Vec<T>> {
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

// #[tracing::instrument]
// pub fn make_grid<T>(input: &str) -> Grid<T> {
//     let mut grid: Grid<T> = Grid::new(vec![]);
//     input.lines().for_each(|line| {
//         grid.push(line.chars().collect());
//     });
//     grid
// }
