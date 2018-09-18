use std::cmp::{Eq, PartialEq};
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Vector2 {
        Vector2 { x: x, y: y }
    }

    pub fn from_array(values: [f32; 2]) -> Vector2 {
        Vector2 {
            x: values[0],
            y: values[1],
        }
    }

    /// Get a vector orthogonal to `self`
    pub fn orthogonal(&self) -> Vector2 {
        Vector2::new(-self.y, self.x)
    }

    pub fn length(&self) -> f32 {
        self.dot(self).sqrt()
    }

    /// Calculate dot product
    pub fn dot(&self, other: &Vector2) -> f32 {
        (self.x * other.x + self.y * other.y)
    }

    /// Subtract a vector from `self`
    pub fn sub(&self, other: &Vector2) -> Vector2 {
        Vector2::new(self.x - other.x, self.y - other.y)
    }

    pub fn get(&self, i: usize) -> f32 {
        if i == 0 {
            self.x
        } else if i == 1 {
            self.y
        } else {
            panic!("Vector index out of range!")
        }
    }

    pub fn set(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
}

impl PartialEq for Vector2 {
    fn eq(&self, other: &Vector2) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Vector2 {}

impl Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Vector2 {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub<Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Vector2 {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<Vector2> for f32 {
    type Output = Vector2;

    fn mul(self, rhs: Vector2) -> Vector2 {
        Vector2::new(self * rhs.x, self * rhs.y)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple_add() {
        let lhs = Vector2::new(0., 0.);
        let rhs = Vector2::new(1., 3.);

        let result = lhs + rhs;

        assert_eq!(result.x, 1.0);
        assert_eq!(result.y, 3.0);
    }

    #[test]
    fn test_simple_sub() {
        let lhs = Vector2::new(0., 0.);
        let rhs = Vector2::new(1., 3.);

        let result = lhs - rhs;

        assert_eq!(result.x, -1.0);
        assert_eq!(result.y, -3.0);
    }

    #[test]
    fn test_simple_scalar_mul() {
        let vec = Vector2::new(2.0, 3.0);
        let result = 0.5 * vec;

        assert_eq!(result.x, 1.0);
        assert_eq!(result.y, 1.5);
    }

    #[test]
    fn test_dot_product() {
        let lhs = Vector2::new(1., 2.);
        let rhs = Vector2::new(1., 3.);

        let result = lhs.dot(&rhs);

        assert_eq!(result, 7.0);
    }

    #[test]
    fn test_length() {
        let vec = Vector2::new(-3., 4.);

        let result = vec.length();

        assert_eq!(result, 5.);
    }
}
