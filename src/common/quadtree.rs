pub use super::aabb::AABB;
pub use super::vector2::Vector2;
use std::mem;

pub enum QuadtreeError {
    OutOfBounds,
    Unknown,
}

const CAPACITY: usize = 4;

pub trait Spacial {
    fn position(&self) -> &Vector2;
}

///
pub struct Quadtree<T>
where
    T: Spacial,
{
    boundary: AABB,
    points: [T; CAPACITY],
    len: usize,
    children: Option<[Box<Quadtree<T>>; 4]>,
}

impl<T> Quadtree<T>
where
    T: Spacial,
{
    pub fn new(boundary: AABB) -> Self {
        let result: Self;
        unsafe {
            result = Self {
                boundary: boundary,
                points: mem::uninitialized(),
                len: 0,
                children: None,
            }
        }
        result
    }

    /// Insert a single element into the tree
    pub fn insert(&mut self, point: T) -> Result<(), QuadtreeError> {
        let assertion = self.can_insert(&point);
        if assertion.is_err() {
            return assertion;
        }

        if self.len < CAPACITY {
            self.points[self.len] = point;
            self.len += 1;
            return Ok(());
        }

        if self.children.is_none() {
            self.subdivide();
        }

        let vessel = self
            .children
            .as_mut()
            .unwrap()
            .iter_mut()
            .find(|child| child.can_insert(&point).is_ok());

        match vessel {
            Some(vessel) => vessel.insert(point),
            None => {
                // This should not happen
                return Err(QuadtreeError::Unknown);
            }
        }
    }

    /// Check if `point` can be inserted into the tree
    pub fn can_insert(&self, point: &T) -> Result<(), QuadtreeError> {
        if !self.boundary.contains(point.position()) {
            Err(QuadtreeError::OutOfBounds)
        } else {
            Ok(())
        }
    }

    /// Insert a vector of elements
    pub fn insert_many(&mut self, points: Vec<T>) -> Result<(), QuadtreeError> {
        for point in points {
            let result = self.insert(point);
            if result.is_err() {
                return result;
            }
        }
        Ok(())
    }

    fn subdivide(&mut self) {
        let current_radius = self.boundary.get_radius();
        let offset = 0.5 * current_radius.clone();
        let center = self.boundary.get_center();
        self.children = Some([
            Box::new(Quadtree::new(AABB::new(
                Vector2::new(center.x - offset.x, center.y + offset.y),
                current_radius.x,
                current_radius.y,
            ))),
            Box::new(Quadtree::new(AABB::new(
                Vector2::new(center.x + offset.x, center.y + offset.y),
                current_radius.x,
                current_radius.y,
            ))),
            Box::new(Quadtree::new(AABB::new(
                Vector2::new(center.x + offset.x, center.y - offset.y),
                current_radius.x,
                current_radius.y,
            ))),
            Box::new(Quadtree::new(AABB::new(
                Vector2::new(center.x - offset.x, center.y - offset.y),
                current_radius.x,
                current_radius.y,
            ))),
        ]);
    }

    /// Queries a `range`, returns a `Vec` of references to points inside `range`
    pub fn query_range<'a>(&'a self, range: AABB) -> Vec<&'a T> {
        let mut result = vec![];
        self.query_range_static(&range, &mut result);
        result
    }

    /// Queries a given range and inserts the matching elements into the `result`
    /// Might be useful if you want to avoid repeated memory allocation
    pub fn query_range_static<'a>(&'a self, range: &AABB, result: &mut Vec<&'a T>) {
        if !range.intersects(&self.boundary) {
            return;
        }

        for i in 0..self.len {
            let p = &self.points[i];
            if range.contains(p.position()) {
                result.push(p);
            }
        }

        if self.children.is_none() {
            return;
        }

        self.children.as_ref().unwrap().iter().for_each(|child| {
            child.query_range_static(range, result);
        });
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;
    use test::Bencher;

    #[derive(Debug, Clone)]
    struct SimpleType {
        pos: Vector2,
        value: u32,
    }

    impl Spacial for SimpleType {
        fn position(&self) -> &Vector2 {
            &self.pos
        }
    }

    #[test]
    fn test_can_create() {
        let boundary = AABB::new(Vector2::new(0., 0.), 50., 40.);
        Quadtree::<SimpleType>::new(boundary);
    }

    #[test]
    fn test_insert_returns_err_if_point_is_out_of_bounds() {
        let boundary = AABB::new(Vector2::new(0., 0.), 50., 40.);
        let mut tree = Quadtree::<SimpleType>::new(boundary);

        let result = tree.insert(SimpleType {
            pos: Vector2::new(26., 0.),
            value: 0,
        });

        assert!(result.is_err());

        match result.unwrap_err() {
            QuadtreeError::OutOfBounds => {}
            _ => panic!("Unexpected error!"),
        }
    }

    #[test]
    fn test_can_insert_many() {
        let boundary = AABB::new(Vector2::new(0., 0.), 50., 40.);
        let mut tree = Quadtree::<SimpleType>::new(boundary);

        let points = vec![
            SimpleType {
                pos: Vector2::new(20., 0.),
                value: 0,
            },
            SimpleType {
                pos: Vector2::new(0., 20.),
                value: 1,
            },
            SimpleType {
                pos: Vector2::new(10., 0.),
                value: 2,
            },
            SimpleType {
                pos: Vector2::new(20., -10.),
                value: 3,
            },
            SimpleType {
                pos: Vector2::new(0., 10.),
                value: 4,
            },
            SimpleType {
                pos: Vector2::new(-20., 5.),
                value: 5,
            },
            SimpleType {
                pos: Vector2::new(5., 0.),
                value: 6,
            },
            SimpleType {
                pos: Vector2::new(10., 10.),
                value: 7,
            },
        ];

        let result = tree.insert_many(points);

        assert!(result.is_ok())
    }

    #[test]
    fn test_can_find_single_element() {
        let boundary = AABB::new(Vector2::new(0., 0.), 50., 40.);
        let mut tree = Quadtree::<SimpleType>::new(boundary);

        let result = tree.insert(SimpleType {
            pos: Vector2::new(20., 0.),
            value: 0,
        });

        assert!(result.is_ok());

        let result = tree.query_range(AABB::new(Vector2::new(10., 0.), 22., 2.));

        assert!(result.len() == 1);
        assert_eq!(result[0].value, 0);
    }

    #[test]
    fn test_returns_empty_if_none_match() {
        let boundary = AABB::new(Vector2::new(0., 0.), 50., 40.);
        let mut tree = Quadtree::<SimpleType>::new(boundary);

        let result = tree.insert(SimpleType {
            pos: Vector2::new(20., 0.),
            value: 0,
        });

        assert!(result.is_ok());

        let result = tree.query_range(AABB::new(Vector2::new(-10., 0.), 22., 2.));

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_can_find_correct_elements() {
        let boundary = AABB::new(Vector2::new(0., 0.), 50., 40.);
        let mut tree = Quadtree::<SimpleType>::new(boundary);

        let points = vec![
            SimpleType {
                pos: Vector2::new(20., 0.),
                value: 0,
            },
            SimpleType {
                pos: Vector2::new(0., 20.),
                value: 0,
            },
            SimpleType {
                pos: Vector2::new(10., 0.),
                value: 1,
            },
            SimpleType {
                pos: Vector2::new(20., -10.),
                value: 0,
            },
            SimpleType {
                pos: Vector2::new(10., 5.),
                value: 1,
            },
            SimpleType {
                pos: Vector2::new(-20., 5.),
                value: 0,
            },
            SimpleType {
                pos: Vector2::new(5., 0.),
                value: 0,
            },
            SimpleType {
                pos: Vector2::new(10., 10.),
                value: 1,
            },
        ];

        let result = tree.insert_many(points);

        assert!(result.is_ok());

        let results = tree.query_range(AABB::new(Vector2::new(10., 0.), 2., 20.));

        println!("{:?}", results);

        assert_eq!(results.len(), 3);
        results.iter().for_each(|result| {
            assert_eq!(result.value, 1);
        });
    }

    #[bench]
    fn bench_query_speed_at_1024_elements(bencher: &mut Bencher) {
        let tree = init_benchmark(1024);

        let mut rng = thread_rng();
        bencher.iter(|| {
            let range = AABB::new(
                Vector2::new(
                    rng.gen_range::<f32>(-50., 50.),
                    rng.gen_range::<f32>(-50., 50.),
                ),
                rng.gen_range::<f32>(5., 100.),
                rng.gen_range::<f32>(5., 100.),
            );

            tree.query_range(range);
        });
    }

    #[bench]
    fn bench_static_query_speed_at_1024_elements(bencher: &mut Bencher) {
        let tree = init_benchmark(1024);

        let mut rng = thread_rng();
        let mut result = vec![];
        bencher.iter(|| {
            result.clear();
            let range = AABB::new(
                Vector2::new(
                    rng.gen_range::<f32>(-50., 50.),
                    rng.gen_range::<f32>(-50., 50.),
                ),
                rng.gen_range::<f32>(5., 100.),
                rng.gen_range::<f32>(5., 100.),
            );

            tree.query_range_static(&range, &mut result);
        });
    }

    fn init_benchmark(size: usize) -> Quadtree<SimpleType> {
        let rng = thread_rng();
        let mut elements = vec![];
        let mut rng = thread_rng();
        for _ in 0..size {
            elements.push(SimpleType {
                pos: Vector2::new(
                    rng.gen_range::<f32>(-50., 50.),
                    rng.gen_range::<f32>(-50., 50.),
                ),
                value: 0,
            });
        }
        let mut tree = Quadtree::new(AABB::new(Vector2::new(0., 0.), 100., 100.));
        let result = tree.insert_many(elements);
        assert!(result.is_ok());
        tree
    }
}
