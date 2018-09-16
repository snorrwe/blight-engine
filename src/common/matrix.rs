use std::mem;

trait WithData {
    type Data;
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
    };
}

matrix!(2, 2, f32, Matrix22);
matrix!(3, 3, f32, Matrix33);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2by2_creation() {
        let data = [0., 1., 2., 3.];
        Matrix22::new(data);
    }
}
