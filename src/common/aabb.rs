use super::vector2::Vector2;

pub struct AABB {
    center: Vector2,
    radius: Vector2,
}

impl AABB {
    pub fn new(center: Vector2, width: f32, height: f32) -> AABB {
        assert!(width > 0.);
        assert!(height > 0.);
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
}
