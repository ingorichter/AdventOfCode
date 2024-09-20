use std::fmt;

use super::point::Point2D;

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
        if !is_safe(point, self) {
            return None;
        }
        Some(&self.grid[point.y as usize][point.x as usize])
    }

    pub fn set(&mut self, point: &Point2D, value: T) {
        if !is_safe(point, self) {
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
        (0..self.columns()).map(move |x| {
            (0..self.rows()).map(move |y| &self.grid[y][x]).collect()
        })
    }

    pub fn insert_row(&mut self, index: usize, row: Vec<T>) {
        self.grid.insert(index, row);
    }

    // pub fn insert_column(&mut self, index: usize, column: Vec<T>) where T: Clone {
    //     let new_column = column.clone();
    //     (0..self.rows()).for_each(|y| self.grid[y].insert(index, new_column[y]));
    // }

    pub fn insert_column(&mut self, index: usize, column: Vec<T>) where T: Clone {
        let column_iter = column.iter().cloned();
        (0..self.rows()).zip(column_iter).for_each(|(y, value)| self.grid[y].insert(index, value));
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.grid.iter().flat_map(|row| row.iter())
    }
}

pub fn is_safe<T>(point: &Point2D, grid: &Grid<T>) -> bool {
    if point.x < 0 || point.x >= grid.columns() as i32 {
        return false;
    }
    if point.y < 0 || point.y >= grid.rows() as i32 {
        return false;
    }
    true
}

// #[tracing::instrument]
// pub fn make_grid<T>(input: &str) -> Grid<T> {
//     let mut grid: Grid<T> = Grid::new(vec![]);
//     input.lines().for_each(|line| {
//         grid.push(line.chars().collect());
//     });
//     grid
// }
