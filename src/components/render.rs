use super::super::systems::render::{RenderComponentInner, RenderSystem};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone)]
pub struct RenderComponent<'a> {
    id: usize,
    system: *mut RenderSystem<'a>,
}

impl<'a> RenderComponent<'a> {
    pub fn new(id: usize, system: *mut RenderSystem<'a>) -> Self {
        RenderComponent {
            id: id,
            system: system,
        }
    }
}

impl<'a> DerefMut for RenderComponent<'a> {
    fn deref_mut(&mut self) -> &mut RenderComponentInner<'a> {
        unsafe { (*self.system).get_component_by_id(self.id) }
    }
}

impl<'a> Deref for RenderComponent<'a> {
    type Target = RenderComponentInner<'a>;

    fn deref(&self) -> &RenderComponentInner<'a> {
        unsafe { (*self.system).get_component_by_id(self.id) }
    }
}

impl<'a> Drop for RenderComponent<'a> {
    fn drop(&mut self) {
        unsafe {
            (*self.system).delete_components_by_ids(&[self.id]);
        }
    }
}
