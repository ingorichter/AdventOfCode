use std::ops;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
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

    pub fn manhattan_distance(&self, other: &Point2D) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn rotate_clockwise(&self) -> Point2D {
        match *self {
            NORTH => EAST,
            EAST => SOUTH,
            SOUTH => WEST,
            WEST => NORTH,
            _ => panic!("Invalid rotation"),
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_points() {
        let point = Point2D::new(1, 2);
        let other = Point2D::new(3, 4);
        let result = point.add(&other);
        assert_eq!(result, Point2D::new(4, 6));
    }

    #[test]
    fn points_are_equal() {
        let point = Point2D::new(1, 2);
        let other = Point2D::new(1, 2);
        assert_eq!(point, other);
    }

    #[test]
    fn points_are_not_equal() {
        let point = Point2D::new(1, 2);
        let other = Point2D::new(3, 4);
        assert_ne!(point, other);
    }

    #[test]
    fn manhattan_distance() {
        let point = Point2D::new(1, 2);
        let other = Point2D::new(3, 4);
        assert_eq!(point.manhattan_distance(&other), 4);
    }
}