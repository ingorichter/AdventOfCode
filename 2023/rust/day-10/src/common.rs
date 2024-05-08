use std::ops;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Tile {
    VerticalPipe,
    HorizontalPipe,
    NorthEastBend,
    NorthWestBend,
    SouthWestBend,
    SouthEastBend,
    Ground,
    Start,
}

impl From<char> for Tile {
    fn from(c: char) -> Tile {
        match c {
            '|' => Tile::VerticalPipe,
            '-' => Tile::HorizontalPipe,
            'L' => Tile::NorthEastBend,
            'J' => Tile::NorthWestBend,
            '7' => Tile::SouthWestBend,
            'F' => Tile::SouthEastBend,
            '.' => Tile::Ground,
            'S' => Tile::Start,
            _ => panic!("Invalid tile type"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point2D {
    pub x: i32,
    pub y: i32,
}

pub const NORTH: Point2D = Point2D { x: 0, y: -1 };
pub const SOUTH: Point2D = Point2D { x: 0, y: 1 };
pub const WEST: Point2D = Point2D { x: -1, y: 0 };
pub const EAST: Point2D = Point2D { x: 1, y: 0 };

impl Point2D {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn add(&self, other: &Point2D) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn sub(&self, other: &Point2D) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    pub fn neighbors(&self) -> Vec<Point2D> {
        vec![
            self.add(&NORTH),
            self.add(&SOUTH),
            self.add(&WEST),
            self.add(&EAST),
        ]
    }
}

impl ops::Add<Point2D> for Point2D {
    type Output = Point2D;

    fn add(self, other: Point2D) -> Point2D {
        Point2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::Sub<Point2D> for Point2D {
    type Output = Point2D;

    fn sub(self, other: Point2D) -> Point2D {
        Point2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
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

#[derive(Debug)]
pub struct Grid<T> {
    grid: Vec<Vec<T>>,
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

    fn push(&mut self, row: Vec<T>) {
        self.grid.push(row);
    }
}

#[tracing::instrument]
pub fn make_grid(input: &str) -> Grid<Tile> {
    let mut grid: Grid<Tile> = Grid::new(vec![]);
    input.lines().for_each(|line| {
        grid.push(line.chars().map(Tile::from).collect());
    });
    grid
}
