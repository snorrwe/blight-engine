use super::aabb::AABB;
use super::matrix::Matrix22;
use super::vector2::Vector2;
use std::f32::EPSILON;

#[derive(Debug, Clone)]
pub struct OBB {
    center: Vector2,     // Center of OBB
    local: [Vector2; 2], // local x and y-axes
    e: Vector2,          // Positive halfwidth extents of OBB along each axis
}

impl OBB {
    pub fn new(center: Vector2, local: [Vector2; 2], radius: Vector2) -> Self {
        assert!(radius.x >= 0.);
        assert!(radius.y >= 0.);
        OBB {
            center: center,
            local: local,
            e: radius,
        }
    }

    pub fn from_aabb(aabb: AABB) -> Self {
        OBB {
            center: aabb.get_center().clone(),
            local: [Vector2::new(1., 0.), Vector2::new(0., 1.)],
            e: aabb.get_radius().clone(),
        }
    }

    pub fn intersects(&self, other: &OBB) -> bool {
        let mut rotation = Matrix22::uninitialised();
        let mut abs_r = Matrix22::uninitialised();

        // Compute rotation matrix expressing `other` in `self`'s coordinate frame
        for i in 0..2 {
            for j in 0..2 {
                rotation.set(i, j, self.local[i].dot(&other.local[j]));
            }
        }

        let translation = other.center.sub(&self.center);
        // Bring translation vector into `a`'s coordinate frame
        let translation = Vector2::new(
            translation.dot(&self.local[0]),
            translation.dot(&self.local[1]),
        );

        // Compute common subexpressions. Add in an epsilon term to
        // counteract arithmetic errors when two edges are parallel and
        // their cross product is (near) null
        for i in 0..2 {
            for j in 0..2 {
                abs_r.set(i, j, rotation.get(i, j).abs() + EPSILON);
            }
        }

        let mut ra: f32;
        let mut rb: f32;

        // Test axis L = A0, L = A1
        for i in 0..2 {
            ra = self.e.get(i);
            rb = other.e.get(0) * abs_r.get(i, 0) + other.e.get(1) * abs_r.get(i, 1);
            if translation.get(i).abs() > ra + rb {
                return false;
            }
        }

        // Test axis L = B0, L = B1
        for i in 0..2 {
            ra = self.e.get(0) * abs_r.get(0, i) + self.e.get(1) * abs_r.get(1, i);
            rb = other.e.get(i);
            let x = (translation.x * rotation.get(0, i) + translation.y + rotation.get(1, i)).abs();
            if x > ra + rb {
                return false;
            }
        }

        // TODO
        // Test axis L = A0 × B0
        // Test axis L = A0 × B1
        // Test axis L = A1 × B0
        // Test axis L = A1 × B1

        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_obb_intersects_itself() {
        let obb = OBB::new(
            Vector2::new(0.0, 0.0),
            [Vector2::new(1., 1.), Vector2::new(1., 1.)],
            Vector2::new(2., 3.),
        );

        assert!(obb.intersects(&obb));
    }

    #[test]
    fn test_non_rotated_obb_intersection_not_overlapping() {
        let lhs = OBB::from_aabb(AABB::new(Vector2::new(0.0, 0.0), 2., 2.));
        let rhs = OBB::from_aabb(AABB::new(Vector2::new(5.0, 0.0), 2., 2.));

        assert!(!lhs.intersects(&rhs));
        assert!(!rhs.intersects(&lhs));
    }

    #[test]
    fn test_simple_intersection_x() {
        let lhs = AABB::new(Vector2::new(-1., 0.), 2., 1.);
        let rhs = AABB::new(Vector2::new(1., 0.), 2.1, 1.);

        assert!(lhs.intersects(&rhs));
        assert!(rhs.intersects(&lhs));

        let lhs = OBB::from_aabb(lhs);
        let rhs = OBB::from_aabb(rhs);

        assert!(lhs.intersects(&rhs));
        assert!(rhs.intersects(&lhs));
    }

    #[test]
    fn test_simple_intersection_y() {
        let lhs = AABB::new(Vector2::new(0., -1.), 1., 2.);
        let rhs = AABB::new(Vector2::new(0., 1.), 1., 2.1);

        assert!(lhs.intersects(&rhs));
        assert!(rhs.intersects(&lhs));

        let lhs = OBB::from_aabb(lhs);
        let rhs = OBB::from_aabb(rhs);

        assert!(lhs.intersects(&rhs));
        assert!(rhs.intersects(&lhs));
    }

    #[test]
    fn test_simple_intersection_xy() {
        let lhs = AABB::new(Vector2::new(1., -1.), 2.1, 2.);
        let rhs = AABB::new(Vector2::new(-1., 1.), 2., 2.1);

        assert!(lhs.intersects(&rhs));
        assert!(rhs.intersects(&lhs));

        let lhs = OBB::from_aabb(lhs);
        let rhs = OBB::from_aabb(rhs);

        assert!(lhs.intersects(&rhs));
        assert!(rhs.intersects(&lhs));
    }
}
