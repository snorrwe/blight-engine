use super::*;
use std::ptr;

/// Represents a renderable object
pub struct RenderComponentInner<'a> {
    texture: *const Texture<'a>,
    position: Rect,
    render_system: *mut RenderSystem<'a>,
    id: usize,
}

impl<'a> RenderComponentInner<'a> {
    pub fn new(render_system: *mut RenderSystem<'a>, id: usize) -> RenderComponentInner<'a> {
        RenderComponentInner {
            render_system: render_system,
            texture: ptr::null(),
            position: Rect::new(0, 0, 0, 0),
            id: id,
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub unsafe fn render(&mut self) {
        assert!(!self.texture.is_null());
        (*self.render_system).render_texture(&*self.texture, &self.position);
    }

    pub fn set_position(&mut self, position: Rect) {
        self.position = position;
    }

    pub fn set_texture(&mut self, texture: *const Texture<'a>) {
        self.texture = texture;
    }
}
