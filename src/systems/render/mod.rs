use sdl2::{pixels, rect, render, video, Sdl};
use std::rc::Rc;

pub type Window = video::Window;
pub type Canvas = render::Canvas<Window>;
pub type TextureCreator = render::TextureCreator<video::WindowContext>;
pub type Texture<'a> = render::Texture<'a>;
pub type TexturePtr<'a> = Rc<Box<Texture<'a>>>;
pub type Rect = rect::Rect;
pub type Color = pixels::Color;

pub mod render_component;
mod render_system;
pub use self::render_system::*;
