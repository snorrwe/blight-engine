use super::aabb::AABB;
use super::matrix::Matrix22;
use super::vector2::Vector2;
use std::f32::{EPSILON, MAX};

#[derive(Debug, Clone)]
pub struct OBB2D {
    center: Vector2,     // Center of OBB2D
    local: [Vector2; 2], // local x and y-axes
    extents: Vector2,    // Positive halfwidth extents of OBB2D along each axis
}

impl OBB2D {
    pub fn new(center: Vector2, local: [Vector2; 2], radius: Vector2) -> Self {
        assert!(radius.x >= 0.);
        assert!(radius.y >= 0.);
        OBB2D {
            center: center,
            local: local,
            extents: radius,
        }
    }

    pub fn from_aabb(aabb: &AABB) -> Self {
        OBB2D {
            center: aabb.get_center().clone(),
            local: [Vector2::new(1., 0.), Vector2::new(0., 1.)],
            extents: aabb.get_radius().clone(),
        }
    }

    // Fit an OBB onto given points
    // At least 3 points are required
    // Note that this function has a complexity of O(n2)
    // To avoid surprises the points must be in "clock-wise order"
    // e.g. given the points:
    // (1, 0) (0, 1), (1, 1), (0, 0)
    // a correct ordering would be
    // [(0, 0), (0, 1), (1, 1), (1, 0)]
    /* ```
        let points = [
            Vector2::new(3.0, 1.0),
            Vector2::new(2.0, 2.0),
            Vector2::new(4.0, 4.0),
            Vector2::new(5.0, 3.0),
        ];

        let result = OBB2D::from_points(&points); // Correct

        let points = [
            Vector2::new(3.0, 1.0),
            Vector2::new(5.0, 3.0),
            Vector2::new(4.0, 4.0),
            Vector2::new(2.0, 2.0),
        ];

        let result = OBB2D::from_points(&points); // Incorrect
       ```
     */
    pub fn from_points(points: &[Vector2]) -> Self {
        assert!(points.len() > 3, "Need at least 3 points to fit an OBB");
        let mut min_area = MAX;
        let mut center: Vector2 = Vector2::new(0., 0.);
        let mut local: [Vector2; 2] = [Vector2::new(1., 0.), Vector2::new(0., 1.)];
        let mut width = 0.;
        let mut height = 0.;
        let mut it = points.iter();
        it.next();
        it.zip(points.iter()).for_each(|(i, j)| {
            // Calculate current edge, normalised
            let mut e0 = j.sub(i);
            e0 = (1. / e0.length()) * e0;

            let e1 = e0.orthogonal();

            let mut min0 = 0.;
            let mut max0 = 0.;
            let mut min1 = 0.;
            let mut max1 = 0.;

            for k in points {
                let d = k.sub(j);

                let handle_dot = |min: &mut f32, max: &mut f32, dot: f32| {
                    if dot < *min {
                        *min = dot;
                    }
                    if dot > *max {
                        *max = dot;
                    }
                };

                let mut dot = d.dot(&e0);
                handle_dot(&mut min0, &mut max0, dot);

                dot = d.dot(&e1);
                handle_dot(&mut min1, &mut max1, dot);
            }
            let area = (max0 - min0) * (max1 - min1);
            if area < min_area {
                min_area = area;
                let l0 = min0 + max0;
                let l1 = min1 + max1;
                center = j.clone() + 0.5 * (l0 * e0.clone() + l1 * e1.clone());
                local[0] = e0;
                local[1] = e1;
                width = l0.abs() * 0.5;
                height = l1.abs() * 0.5;
            }
        });
        OBB2D::new(center, local, Vector2::new(width, height))
    }

    pub fn get_center(&self) -> &Vector2 {
        &self.center
    }

    pub fn get_local(&self) -> &[Vector2] {
        &self.local
    }

    pub fn get_extents(&self) -> &Vector2 {
        &self.extents
    }

    // Note that this method creates a new Matrix on each call
    // Use `get_local` if you want to avoid copying
    pub fn rotation_matrix(&self) -> Matrix22 {
        Matrix22::new([
            self.local[0].x,
            self.local[0].y,
            self.local[1].x,
            self.local[1].y,
        ])
    }

    pub fn intersects_aabb(&self, other: &AABB) -> bool {
        let other = OBB2D::from_aabb(other);
        self.intersects(&other)
    }

    pub fn intersects(&self, other: &OBB2D) -> bool {
        let mut rotation = Matrix22::uninitialised();
        let mut abs_rot = Matrix22::uninitialised();

        // Compute rotation matrix expressing `other` in `self`'s coordinate
        // frame
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
                abs_rot.set(i, j, rotation.get(i, j).abs() + EPSILON);
            }
        }

        let mut ra: f32;
        let mut rb: f32;

        // Test axis L = A0, L = A1
        for i in 0..2 {
            ra = self.extents.get(i);
            rb =
                other.extents.get(0) * abs_rot.get(i, 0) + other.extents.get(1) * abs_rot.get(i, 1);
            let x = translation.get(i).abs();
            if x > ra + rb {
                return false;
            }
        }

        // Test axis L = B0, L = B1
        for i in 0..2 {
            const Z_AXIS_INVARIANT: f32 = 1.0;
            ra = self.extents.x * abs_rot.get(0, i)
                + self.extents.y * abs_rot.get(1, i)
                + Z_AXIS_INVARIANT;
            rb = other.extents.get(i);
            let x = translation.x * rotation.get(0, i) + translation.y + rotation.get(1, i);
            if x.abs() > ra + rb {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_obb_intersects_itself() {
        let obb = OBB2D::new(
            Vector2::new(0.0, 0.0),
            [Vector2::new(1., 1.), Vector2::new(1., 1.)],
            Vector2::new(2., 3.),
        );

        assert!(obb.intersects(&obb));
    }

    #[test]
    fn test_non_rotated_obb_intersection_not_overlapping() {
        let lhs = OBB2D::from_aabb(&AABB::new(Vector2::new(0.0, 0.0), 2., 2.));
        let rhs = OBB2D::from_aabb(&AABB::new(Vector2::new(5.0, 0.0), 2., 2.));

        assert!(!lhs.intersects(&rhs));
        assert!(!rhs.intersects(&lhs));
    }

    #[test]
    fn test_simple_intersection_x() {
        let lhs = AABB::new(Vector2::new(-1., 0.), 2., 1.);
        let rhs = AABB::new(Vector2::new(1., 0.), 2.1, 1.);

        assert!(lhs.intersects(&rhs));
        assert!(rhs.intersects(&lhs));

        let lhs = OBB2D::from_aabb(&lhs);
        let rhs = OBB2D::from_aabb(&rhs);

        assert!(lhs.intersects(&rhs));
        assert!(rhs.intersects(&lhs));
    }

    #[test]
    fn test_simple_intersection_y() {
        let lhs = AABB::new(Vector2::new(0., -1.), 1., 2.);
        let rhs = AABB::new(Vector2::new(0., 1.), 1., 2.1);

        assert!(lhs.intersects(&rhs));
        assert!(rhs.intersects(&lhs));

        let lhs = OBB2D::from_aabb(&lhs);
        let rhs = OBB2D::from_aabb(&rhs);

        assert!(lhs.intersects(&rhs));
        assert!(rhs.intersects(&lhs));
    }

    #[test]
    fn test_simple_intersection_xy() {
        let lhs = AABB::new(Vector2::new(1., -1.), 2.1, 2.);
        let rhs = AABB::new(Vector2::new(-1., 1.), 2., 2.1);

        assert!(lhs.intersects(&rhs));
        assert!(rhs.intersects(&lhs));

        let lhs = OBB2D::from_aabb(&lhs);
        let rhs = OBB2D::from_aabb(&rhs);

        assert!(lhs.intersects(&rhs));
        assert!(rhs.intersects(&lhs));
    }

    #[test]
    fn test_simple_non_intersecting() {
        let lhs = AABB::new(Vector2::new(-1., 0.), 1., 1.);
        let rhs = AABB::new(Vector2::new(1., 0.), 1., 1.);

        assert!(!lhs.intersects(&rhs));
        assert!(!rhs.intersects(&lhs));

        let lhs = OBB2D::from_aabb(&lhs);
        let rhs = OBB2D::from_aabb(&rhs);

        assert!(!lhs.intersects(&rhs));
        assert!(!rhs.intersects(&lhs));
    }

    #[test]
    fn test_simple_rotated_non_intersecting() {
        let lhs = OBB2D::from_points(&[
            Vector2::new(3.0, 1.0),
            Vector2::new(2.0, 2.0),
            Vector2::new(4.0, 4.0),
            Vector2::new(5.0, 3.0),
        ]);

        let rhs = OBB2D::from_points(&[
            Vector2::new(6.0, 1.0),
            Vector2::new(8.0, 1.0),
            Vector2::new(8.0, 3.0),
            Vector2::new(6.0, 3.0),
        ]);

        assert!(!lhs.intersects(&rhs));
        assert!(!rhs.intersects(&lhs));
    }

    #[test]
    fn test_simple_point_fitting() {
        let points = [
            Vector2::new(3.0, 1.0),
            Vector2::new(2.0, 2.0),
            Vector2::new(4.0, 4.0),
            Vector2::new(5.0, 3.0),
        ];

        let result = OBB2D::from_points(&points);

        assert_eq!(*result.get_center(), Vector2::new(3.5, 2.5));
        assert_eq!(
            *result.get_local(),
            [
                Vector2::new(0.70710677, -0.70710677),
                Vector2::new(0.70710677, 0.70710677)
            ]
        );
        assert_eq!(*result.get_extents(), Vector2::new(0.70710677, 1.4142135));

        let result = OBB2D::from_points(&[
            Vector2::new(6.0, 3.0),
            Vector2::new(8.0, 3.0),
            Vector2::new(8.0, 1.0),
            Vector2::new(6.0, 1.0),
        ]);

        assert_eq!(*result.get_center(), Vector2::new(7., 2.));
        assert_eq!(
            *result.get_local(),
            [Vector2::new(-1., 0.), Vector2::new(0., -1.)]
        );
        assert_eq!(*result.get_extents(), Vector2::new(1., 1.));
    }

    #[test]
    fn test_simple_rotated_intersecting() {
        let lhs = OBB2D::from_points(&[
            Vector2::new(4.18, 0.3),
            Vector2::new(3.18, 1.3),
            Vector2::new(5.18, 3.3),
            Vector2::new(6.18, 2.3),
        ]);

        let rhs = OBB2D::from_points(&[
            Vector2::new(6.0, 1.0),
            Vector2::new(8.0, 1.0),
            Vector2::new(8.0, 3.0),
            Vector2::new(6.0, 3.0),
        ]);

        assert!(lhs.intersects(&rhs));
        assert!(rhs.intersects(&lhs));
    }

    #[test]
    #[should_panic]
    fn test_too_few_points_panic() {
        let points = [Vector2::new(3.0, 1.0), Vector2::new(5.0, 3.0)];
        let _result = OBB2D::from_points(&points);
    }
}
