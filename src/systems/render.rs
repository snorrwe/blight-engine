use sdl2::pixels::Color;
use sdl2::{rect, render, video, Sdl};
use std::rc::Rc;

pub type Window = video::Window;
pub type Canvas = render::Canvas<Window>;
pub type TextureCreator = render::TextureCreator<video::WindowContext>;
pub type Texture<'a> = render::Texture<'a>;
pub type TexturePtr<'a> = Rc<Box<Texture<'a>>>;
pub type Rect = rect::Rect;

pub const WINDOW_SIZE: (u32, u32) = (800, 600); // TODO

pub struct RenderSystem {
    canvas: Canvas,
    texture_creator: TextureCreator,
    background_color: Color,
}

pub enum VideoError {
    NotInitialised,
}

impl RenderSystem {
    pub fn new(sdl_context: &Sdl) -> RenderSystem {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("Blight Engine", WINDOW_SIZE.0, WINDOW_SIZE.1)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        let result = RenderSystem {
            texture_creator: canvas.texture_creator(),
            canvas: canvas,
            background_color: Color::RGB(0, 0, 0),
        };
        result
    }

    pub fn get_canvas(&self) -> &Canvas {
        &self.canvas
    }

    pub fn get_canvas_mut(&mut self) -> &mut Canvas {
        &mut self.canvas
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(self.background_color);
        self.canvas.clear();
    }

    pub fn set_background_color(&mut self, background: Option<Color>) {
        self.background_color = background.unwrap_or(Color::RGB(0, 0, 0));
    }

    pub fn render(&mut self) {
        self.canvas.present();
    }

    pub fn texture_creator<'a>(&'a self) -> &'a TextureCreator {
        &self.texture_creator
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
