use sdl2::pixels::Color;
use sdl2::{rect, render, video, Sdl};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use super::super::components::render::RenderComponent as RenderComponentInner;

pub type Window = video::Window;
pub type Canvas = render::Canvas<Window>;
pub type TextureCreator = render::TextureCreator<video::WindowContext>;
pub type Texture<'a> = render::Texture<'a>;
pub type TexturePtr<'a> = Rc<Box<Texture<'a>>>;
pub type Rect = rect::Rect;

pub const WINDOW_SIZE: (u32, u32) = (800, 600); // TODO

#[derive(Debug, Clone)]
pub struct RenderComponent<'a> {
    id: usize,
    system: *mut RenderSystem<'a>,
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

pub struct RenderSystem<'a> {
    canvas: Canvas,
    texture_creator: TextureCreator,
    background_color: Color,
    render_components: Vec<RenderComponentInner<'a>>,
}

pub enum VideoError {
    NotInitialised,
}

impl<'a> RenderSystem<'a> {
    pub fn new(sdl_context: &Sdl, window_size: Option<(u32, u32)>) -> RenderSystem<'a> {
        let window_size = match window_size {
            Some(x) => x,
            None => WINDOW_SIZE,
        };
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("Blight Engine", window_size.0, window_size.1)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        RenderSystem {
            texture_creator: canvas.texture_creator(),
            canvas: canvas,
            background_color: Color::RGB(0, 0, 0),
            render_components: vec![],
        }
    }

    pub fn get_canvas(&self) -> &Canvas {
        &self.canvas
    }

    pub fn get_canvas_mut(&mut self) -> &mut Canvas {
        &mut self.canvas
    }

    pub fn set_background_color(&mut self, background: Option<Color>) {
        self.background_color = background.unwrap_or(Color::RGB(0, 0, 0));
    }

    pub fn render(&mut self) {
        self.clear();
        self.render_components
            .iter_mut()
            .for_each(|component| unsafe {
                component.render();
            });
        self.canvas.present();
    }

    fn clear(&mut self) {
        self.canvas.set_draw_color(self.background_color);
        self.canvas.clear();
    }

    /// Borrow the render system's texture creator
    pub fn texture_creator(&'a self) -> &'a TextureCreator {
        &self.texture_creator
    }

    pub fn render_texture(&mut self, texture: &Texture<'a>, rect: &Rect) {
        self.canvas.copy(texture, None, *rect).unwrap();
    }

    pub fn create_texture(&'a mut self, size: &(u32, u32)) -> Texture {
        let texture = self
            .texture_creator()
            .create_texture_target(None, size.0, size.1)
            .unwrap();
        texture
    }

    pub fn create_component(&'a mut self) -> RenderComponent {
        static mut NEXT_ID: usize = 0;
        unsafe {
            assert!(NEXT_ID < <usize>::max_value());
            NEXT_ID += 1;
            let result = RenderComponentInner::new(self as *mut RenderSystem<'a>, NEXT_ID.clone());
            self.render_components.push(result);
            RenderComponent {
                id: NEXT_ID.clone(),
                system: self as *mut RenderSystem,
            }
        }
    }

    pub fn get_components_by_ids(
        &'a mut self,
        ids: &[usize],
    ) -> Vec<&mut RenderComponentInner<'a>> {
        self.render_components
            .iter_mut()
            .filter(|component| {
                let id = component.get_id();
                ids.iter().any(|i| *i == id)
            })
            .collect()
    }

    pub fn get_component_by_id(&'a mut self, id: usize) -> &mut RenderComponentInner<'a> {
        self.render_components
            .iter_mut()
            .find(|component| component.get_id() == id)
            .expect(&format!("No component exists by the id [{}]", id))
    }

    pub fn delete_components_by_ids(&mut self, ids: &[usize]) {
        self.render_components.retain(|component| {
            let id = component.get_id();
            ids.iter().any(|i| *i != id)
        });
    }

    pub fn clear_components(&mut self) {
        self.render_components.clear();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;
    use sdl2;
    use test::Bencher;

    #[bench]
    fn simple_render_bunch<'a>(bencher: &mut Bencher) {
        let sdl = sdl2::init().unwrap();
        let mut render_system = RenderSystem::new(&sdl, None);
        let render_ptr = &mut render_system as *mut RenderSystem;
        unsafe {
            let mut components = vec![];
            const TEXTURE_SIZE: u32 = 50;
            let mut texture = (*render_ptr).create_texture(&(TEXTURE_SIZE, TEXTURE_SIZE));

            let mut rng = thread_rng();
            for _ in 0..100 {
                let mut component = (*render_ptr).create_component();
                component.set_texture(&mut texture);
                let x = rng.gen_range::<i32>(50, 500);
                let y = rng.gen_range::<i32>(50, 500);
                component.set_position(Rect::new(x, y, TEXTURE_SIZE, TEXTURE_SIZE));
                components.push(component);
            }

            bencher.iter(|| {
                render_system.render();
            })
        }
    }
}
