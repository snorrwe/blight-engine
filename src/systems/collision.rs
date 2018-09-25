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
}

impl<'a> CollisionSystem<'a> {
    pub fn new(boundary: AABB) -> Self {
        Self {
            components: vec![],
            world: Quadtree::new(boundary),
        }
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
