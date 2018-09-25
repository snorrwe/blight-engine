use super::*;
use std::ptr;

/// Represents a renderable object
pub struct RenderComponentInner<'a> {
    pub texture: *const Texture<'a>,
    pub position: Rect,
    pub id: usize,
}

impl<'a> RenderComponentInner<'a> {
    pub fn new(id: usize) -> RenderComponentInner<'a> {
        RenderComponentInner {
            texture: ptr::null(),
            position: Rect::new(0, 0, 0, 0),
            id: id,
        }
    }
}
