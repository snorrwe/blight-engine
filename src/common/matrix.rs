use std::cmp;
use std::f32::consts;
use std::mem;

trait WithData {
    type Data;
}

trait Rotation {
    fn from_radians(rad: f32) -> Self;
    fn from_degrees(degrees: f32) -> Self;
}

macro_rules! matrix {
    ($c:expr, $r:expr, $data:ty, $name:ident) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            data: [$data; $c * $r],
        }

        impl WithData for $name {
            type Data = $data;
        }

        impl $name {
            const COLUMNS: usize = $c;
            const ROWS: usize = $r;
            const SIZE: usize = $c * $r;

            pub fn new(data: [$data; $c * $r]) -> Self {
                $name { data: data }
            }

            pub fn uninitialised() -> Self {
                let data: [$data; Self::SIZE];
                unsafe {
                    data = mem::uninitialized();
                }
                $name { data: data }
            }

            pub fn get(&self, col: usize, row: usize) -> &$data {
                assert!(col < Self::COLUMNS);
                assert!(row < Self::ROWS);
                &self.data[col * Self::COLUMNS + row]
            }

            pub fn get_mut(&mut self, col: usize, row: usize) -> &mut $data {
                assert!(col < Self::COLUMNS);
                assert!(row < Self::ROWS);
                &mut self.data[col * Self::COLUMNS + row]
            }

            pub fn set(&mut self, col: usize, row: usize, value: $data) {
                assert!(col < Self::COLUMNS);
                assert!(row < Self::ROWS);
                self.data[col * Self::COLUMNS + row] = value;
            }
        }

        impl cmp::PartialEq for $name {
            fn eq(&self, other: &$name) -> bool {
                !self
                    .data
                    .iter()
                    .zip(other.data.iter())
                    .any(|(&x, &y)| x != y)
            }
        }

        impl cmp::Eq for $name {}
    };
}

matrix!(2, 2, f32, Matrix22);
matrix!(3, 3, f32, Matrix33);

impl Rotation for Matrix22 {
    fn from_radians(rad: f32) -> Self {
        Matrix22::new([rad.cos(), -rad.sin(), rad.sin(), rad.cos()])
    }

    fn from_degrees(degrees: f32) -> Self {
        Self::from_radians(degrees * consts::PI / 180.)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2by2_creation() {
        let data = [0., 1., 2., 3.];
        Matrix22::new(data);
    }

    #[test]
    fn test_equality() {
        let data = [0., 1., 2., 3.];
        let lhs = Matrix22::new(data.clone());
        let rhs = Matrix22::new(data.clone());
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_inequality() {
        let data = [0., 1., 2., 3.];
        let lhs = Matrix22::new(data);
        let data = [1., 1., 2., 3.];
        let rhs = Matrix22::new(data);
        assert_ne!(lhs, rhs);
    }

    macro_rules! two_by_two_nearly_eq {
        ($x:expr, $y:expr, $d:expr) => {
            for i in 0..2 {
                for j in 0..2 {
                    assert!(($x.get(i, j) - $y.get(i, j)).abs() < $d);
                }
            }
        };
    }

    #[test]
    fn test_2by2_rotation() {
        const MAX_DIFF: f32 = 0.000005;

        let matrix = Matrix22::from_radians(consts::PI);
        let expected = Matrix22::new([-1.0, 0.0, 0.0, -1.0]);

        two_by_two_nearly_eq!(matrix, expected, MAX_DIFF);

        let matrix = Matrix22::from_radians(consts::PI * 0.5);
        let expected = Matrix22::new([0.0, -1.0, 1.0, 0.0]);

        two_by_two_nearly_eq!(matrix, expected, MAX_DIFF);
    }
}
