use super::vector2::Vector2;

pub struct AABB {
    center: Vector2,
    radius: Vector2,
}

impl AABB {
    fn new(center: Vector2, width: f32, height: f32) -> AABB {
        AABB {
            center: center,
            radius: Vector2::new(width / 2.0, height / 2.0),
        }
    }
}
