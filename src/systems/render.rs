use std::rc::Rc;
use sdl2::{rect, render, video, Sdl};

pub type Window = video::Window;
pub type Canvas = render::Canvas<Window>;
pub type TextureCreator = render::TextureCreator<video::WindowContext>;
pub type Texture<'a> = render::Texture<'a>;
pub type TexturePtr<'a> = Rc<Box<Texture<'a>>>;
pub type Rect = rect::Rect;

pub struct RenderSystem {
    canvas: Canvas,
    texture_creator: TextureCreator,
}

pub enum VideoError {
    NotInitialised,
}

impl RenderSystem {
    pub fn new(sdl_context: &Sdl) -> RenderSystem {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("Blight Engine", 800, 600)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        let result = RenderSystem {
            texture_creator: canvas.texture_creator(),
            canvas: canvas,
        };
        result
    }

    pub fn get_canvas(&mut self) -> &mut Canvas {
        &mut self.canvas
    }

    pub fn texture_creator<'a>(&'a mut self) -> &'a mut TextureCreator {
        &mut self.texture_creator
    }

    pub fn render_texture<'a>(&mut self, texture: &Texture<'a>, rect: &Rect) {
        self.canvas.copy(texture, None, *rect).unwrap();
    }

    pub fn create_texture<'a>(&'a mut self, size: &(u32, u32)) -> Texture {
        let texture = self
            .texture_creator()
            .create_texture_target(None, size.0, size.1)
            .unwrap();
        texture
    }
}
