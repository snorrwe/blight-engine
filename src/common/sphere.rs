use super::vector2::Vector2;

#[derive(Debug)]
pub struct Sphere {
    center: Vector2,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vector2, radius: f32) -> Self {
        Sphere {
            center: center,
            radius: radius,
        }
    }

    pub fn intersects(&self, other: &Sphere) -> bool {
        let distance = self.center.sub(&other.center);
        let dist_sqrd = distance.dot(&distance);
        let radius_sum = self.radius + other.radius;
        dist_sqrd <= radius_sum * radius_sum
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_spehere_intersects_itself() {
        let sphere = Sphere::new(Vector2::new(0., 0.), 5.);
        assert!(sphere.intersects(&sphere));
    }

    #[test]
    fn test_sphere_sphere_intersection() {
        let lhs = Sphere::new(Vector2::new(0., 0.), 5.);
        let rhs = Sphere::new(Vector2::new(10., 0.), 5.);

        assert!(lhs.intersects(&rhs));
        assert!(rhs.intersects(&lhs));
    }

    #[test]
    fn test_sphere_sphere_too_far_are_not_intersecting() {
        let lhs = Sphere::new(Vector2::new(0., 0.), 5.);
        let rhs = Sphere::new(Vector2::new(10., 0.), 4.);

        assert!(!lhs.intersects(&rhs));
        assert!(!rhs.intersects(&lhs));
    }
}