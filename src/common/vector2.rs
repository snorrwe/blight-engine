use std::ops::{Add, Mul, Sub};

pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Vector2 {
        Vector2 { x: x, y: y }
    }
}

impl Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<Vector2> for f32 {
    type Output = Vector2;

    fn mul(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: rhs.x * self,
            y: rhs.y * self,
        }
    }
}

#[cfg(test)]
mod tests {
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
    fn tesT_simple_scalar_mul() {
        let vec = Vector2::new(2.0, 3.0);
        let result = 0.5 * vec;

        assert_eq!(result.x, 1.0);
        assert_eq!(result.y, 1.5);
    }
}
