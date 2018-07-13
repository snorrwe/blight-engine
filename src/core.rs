use std::cell::Cell;
use std::time::Duration;

use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use super::systems::input::InputSystem;
use super::systems::render::RenderSystem;
use super::Game;

pub struct BlightCore {
    render_system: Box<RenderSystem>,
    input_system: Box<InputSystem>,
    running: Cell<bool>,
}

impl BlightCore {
    pub fn new() -> BlightCore {
        let sdl = sdl2::init().unwrap();
        let render_system = RenderSystem::new(&sdl);
        let input_system = InputSystem::new(&sdl);
        BlightCore {
            render_system: Box::new(render_system),
            input_system: Box::new(input_system),
            running: Cell::new(false),
        }
    }

    pub fn run<TGame>(&mut self, game: &mut TGame)
    where
        TGame: Game,
    {
        self.running.set(true);
        while self.running.get() {
            self.clear_canvas();
            self.update_input();
            game.update();
            self.render_system.get_canvas().present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    pub fn get_input(&self) -> &InputSystem {
        &self.input_system
    }

    pub fn get_render(&mut self) -> *mut RenderSystem {
        &mut *self.render_system as *mut RenderSystem
    }

    fn update_input(&mut self) {
        self.input_system.update();
        self.input_system.handle_events(&|event| match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => self.running.set(false),
            _ => {}
        });
    }

    fn clear_canvas(&mut self) {
        let canvas = self.render_system.get_canvas();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
    }
}
