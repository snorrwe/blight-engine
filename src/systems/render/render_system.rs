pub use super::super::super::components::render::RenderComponent;
pub use super::render_component::RenderComponentInner;
use std::collections::BTreeMap;

use super::*;

pub const WINDOW_SIZE: (u32, u32) = (800, 600);

/// Handles rendering
pub struct RenderSystem<'a> {
    canvas: Canvas,
    texture_creator: TextureCreator,
    background_color: Color,
    render_components: BTreeMap<usize, RenderComponentInner<'a>>,
    next_id: usize,
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
            render_components: BTreeMap::new(),
            next_id: 0,
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

    /// Render all the components in the system
    pub fn render(&mut self) {
        self.clear();
        // Take ownership of `render_components`
        let components = std::mem::replace(&mut self.render_components, BTreeMap::new());
        components.values().for_each(|component| unsafe {
            self.render_texture(&*component.texture, &component.position);
        });
        // Return owrnership to the render system
        self.render_components = components;
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
        assert!(self.next_id < <usize>::max_value());
        self.next_id += 1;
        let result = RenderComponentInner::new(self.next_id);
        self.render_components.insert(self.next_id, result);
        RenderComponent::new(self.next_id, self as *mut RenderSystem)
    }

    pub fn get_components_by_ids(
        &'a mut self,
        ids: &[usize],
    ) -> Vec<&mut RenderComponentInner<'a>> {
        self.render_components
            .values_mut()
            .filter(|component| {
                let id = component.id;
                ids.iter().any(|i| *i == id)
            })
            .collect()
    }

    pub fn get_component_by_id(&'a mut self, id: usize) -> &mut RenderComponentInner<'a> {
        self.render_components
            .get_mut(&id)
            .expect(&format!("No component exists by the id [{}]", id))
    }

    pub fn delete_components_by_ids(&mut self, ids: &[usize]) {
        for id in ids {
            self.render_components.remove(id);
        }
    }

    pub fn purge_components(&mut self) {
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
        render_system.set_background_color(Some(Color::RGB(255, 255, 255)));
        let render_ptr = &mut render_system as *mut RenderSystem;
        unsafe {
            let mut components = vec![];
            const TEXTURE_SIZE: u32 = 50;
            let mut texture = (*render_ptr).create_texture(&(TEXTURE_SIZE, TEXTURE_SIZE));

            let mut rng = thread_rng();
            for _ in 0..100 {
                let mut component = (*render_ptr).create_component();
                component.texture = &mut texture;
                components.push(component);
            }

            bencher.iter(|| {
                components.iter_mut().for_each(|component| {
                    let x = rng.gen_range::<i32>(50, 500);
                    let y = rng.gen_range::<i32>(50, 500);
                    component.position = Rect::new(x, y, TEXTURE_SIZE, TEXTURE_SIZE);
                });
                render_system.render();
            })
        }
    }
}
