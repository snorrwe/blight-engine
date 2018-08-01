use super::vector2::Vector2;

pub struct AABB {
    center: Vector2,
    radius: Vector2,
}

impl AABB {
    pub fn new(center: Vector2, width: f32, height: f32) -> AABB {
        AABB {
            center: center,
            radius: Vector2::new(width / 2.0, height / 2.0),
        }
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
    fn test_simple_intersection() {
        let lhs = AABB::new(Vector2::new(-1., 0.), 2., 1.);
        let rhs = AABB::new(Vector2::new(1., 0.), 2.1, 1.);

        assert!(lhs.intersects(&rhs));
        assert!(rhs.intersects(&lhs));
    }
}
