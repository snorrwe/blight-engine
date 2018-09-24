use super::vector2::Vector2;

/// Axis Aligned Bounding Box
#[derive(Debug, Clone)]
pub struct AABB {
    center: Vector2,
    radius: Vector2,
}

impl AABB {
    pub fn new(center: Vector2, width: f32, height: f32) -> AABB {
        assert!(width > 0.0);
        assert!(height > 0.0);
        AABB {
            center: center,
            radius: Vector2::new(width / 2.0, height / 2.0),
        }
    }

    pub fn get_center<'a>(&'a self) -> &'a Vector2 {
        &self.center
    }

    pub fn get_center_mut<'a>(&'a mut self) -> &'a mut Vector2 {
        &mut self.center
    }

    pub fn get_radius<'a>(&'a self) -> &'a Vector2 {
        &self.radius
    }

    pub fn get_radius_mut<'a>(&'a mut self) -> &'a mut Vector2 {
        &mut self.radius
    }

    pub fn intersects(&self, other: &AABB) -> bool {
        (self.center.x - other.center.x).abs() <= self.radius.x + other.radius.x
            && (self.center.y - other.center.y).abs() <= self.radius.y + other.radius.y
    }

    /// Calculate the closest point on the AABB to the given point
    pub fn closest_point(&self, point: &Vector2) -> Vector2 {
        self.closest_point_mut(point.clone())
    }

    /// Calculate the closest point on the AABB to the given point, consuming the original point
    pub fn closest_point_mut(&self, mut result: Vector2) -> Vector2 {
        let min = self.center.sub(&self.radius);
        let max = self.center.add(&self.radius);

        for i in 0..2 {
            let v = result.get_mut(i);
            if *v < min.get(i) {
                *v = min.get(i).clone();
            }
            if *v > max.get(i) {
                *v = max.get(i).clone();
            }
        }

        result
    }

    pub fn contains(&self, point: &Vector2) -> bool {
        self.closest_point(point) == *point
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_aabb_intersects_itself() {
        let aabb = AABB::new(Vector2::new(-1., 0.), 2., 1.);
        assert!(aabb.intersects(&aabb));
    }

    #[test]
    fn test_simple_intersection_x() {
        let lhs = AABB::new(Vector2::new(-1., 0.), 2., 1.);
        let rhs = AABB::new(Vector2::new(1., 0.), 2.1, 1.);

        assert!(lhs.intersects(&rhs));
        assert!(rhs.intersects(&lhs));
    }

    #[test]
    fn test_simple_intersection_y() {
        let lhs = AABB::new(Vector2::new(0., -1.), 1., 2.);
        let rhs = AABB::new(Vector2::new(0., 1.), 1., 2.1);

        assert!(lhs.intersects(&rhs));
        assert!(rhs.intersects(&lhs));
    }

    #[test]
    fn test_simple_intersection_xy() {
        let lhs = AABB::new(Vector2::new(1., -1.), 2.1, 2.);
        let rhs = AABB::new(Vector2::new(-1., 1.), 2., 2.1);

        assert!(lhs.intersects(&rhs));
        assert!(rhs.intersects(&lhs));
    }

    #[test]
    fn test_simple_non_intersecting() {
        let lhs = AABB::new(Vector2::new(-1., 0.), 1., 1.);
        let rhs = AABB::new(Vector2::new(1., 0.), 1., 1.);

        assert!(!lhs.intersects(&rhs));
        assert!(!rhs.intersects(&lhs));
    }

    #[test]
    #[should_panic]
    fn test_width_has_to_be_positive() {
        AABB::new(Vector2::new(0., 0.), -1., 1.);
    }

    #[test]
    #[should_panic]
    fn test_height_has_to_be_positive() {
        AABB::new(Vector2::new(0., 0.), 1., -1.);
    }

    #[test]
    fn test_closest_points() {
        // tuples of input and expected output
        let test_cases = [
            (Vector2::new(-21., 11.), Vector2::new(-20., 10.)),
            (Vector2::new(-22., 5.), Vector2::new(-20., 5.)),
            (Vector2::new(-21., -11.), Vector2::new(-20., -10.)),
            (Vector2::new(25., -8.), Vector2::new(20., -8.)),
            (Vector2::new(10., 15.), Vector2::new(10., 10.)),
            (Vector2::new(42., 18.), Vector2::new(20., 10.)),
        ];

        let aabb = AABB::new(Vector2::new(0., 0.), 40., 20.);

        for (input, expected) in test_cases.iter() {
            let actual = aabb.closest_point(&input);
            assert_eq!(actual, *expected);
        }
    }

    #[test]
    fn test_closest_point_intersecting_aabb() {
        let aabb = AABB::new(Vector2::new(0., 0.), 40., 20.);
        let actual = aabb.closest_point(&Vector2::new(5., 5.));
        assert_eq!(actual, Vector2::new(5., 5.));
    }
}
