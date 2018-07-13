use super::super::systems::render::{RenderSystem, Texture, TexturePtr};

pub struct RenderComponent<'a> {
    texture: &'a Texture<'a>,
}

impl<'a> RenderComponent<'a> {
    pub fn new(texture: &'a Texture<'a>) -> Self {
        RenderComponent { texture: texture }
    }
}
