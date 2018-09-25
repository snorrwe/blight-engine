use super::super::common::aabb::AABB;
use super::super::common::quadtree::{Quadtree, Spacial};
use super::super::common::vector2::Vector2;

#[derive(Debug, Clone)]
pub enum Collider {
    AABB(AABB),
}

#[derive(Debug, Clone)]
pub struct CollisionComponent<'a> {
    pos: &'a Vector2,
    collider: Collider,
}

impl<'r> Spacial for CollisionComponent<'r> {
    fn position<'a>(&'a self) -> &'a Vector2 {
        &self.pos
    }
}

#[derive(Debug)]
pub struct CollisionSystem<'a> {
    components: Vec<CollisionComponent<'a>>,
    world: Quadtree<CollisionComponent<'a>>,
    center: Vector2,
}

impl<'a> CollisionSystem<'a> {
    pub fn new(boundary: AABB) -> Self {
        Self {
            components: vec![],
            center: boundary.get_center().clone(),
            world: Quadtree::new(boundary),
        }
    }

    /// Sets the center of the world. This does not allocate additional memory
    pub fn set_center(&mut self, position: Vector2) {
        self.center = position;
    }

    /// Sets the boundary of the world, note that this requires reallocating the world's memory
    pub fn set_boundary(&mut self, boundary: AABB) {
        self.world = Quadtree::new(boundary);
    }

    pub fn update(&mut self) {
        self.world.clear();
        self.world
            .insert_many(self.components.iter().map(|component| component.clone()))
            .expect("Component out of world bounds");
    }
}

#[cfg(test)]
mod test {
    // use super::*;
    //
    // #[test]
    // fn test_() {}
}
