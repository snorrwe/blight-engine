use std::cell::Cell;
use std::time::{Duration, Instant};

use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use super::systems::input::InputSystem;
use super::systems::render::RenderSystem;
use super::Game;

/// The hearth of the Blight Engine
/// It's responsible for the game loop
pub struct BlightCore<'a> {
    render_system: Box<RenderSystem<'a>>,
    input_system: Box<InputSystem>,
    running: Cell<bool>,
    delta_time: Duration,
    target_ms_per_update: u64,
}

impl<'a> BlightCore<'a> {
    pub fn new() -> BlightCore<'a> {
        const TARGET_MS_PER_UPDATE: u64 = 1000 / 60;
        let sdl = sdl2::init().unwrap();
        let render_system = RenderSystem::new(&sdl);
        let input_system = InputSystem::new(&sdl);
        BlightCore {
            render_system: Box::new(render_system),
            input_system: Box::new(input_system),
            running: Cell::new(false),
            delta_time: Duration::from_secs(0),
            target_ms_per_update: TARGET_MS_PER_UPDATE,
        }
    }

    pub fn get_delta_time(&self) -> &Duration {
        &self.delta_time
    }

    pub fn get_input(&self) -> &InputSystem {
        &self.input_system
    }

    pub fn get_render(&mut self) -> *mut RenderSystem<'a> {
        &mut *self.render_system as *mut RenderSystem<'a>
    }

    pub fn run<TGame>(&mut self, game: &mut TGame)
    where
        TGame: Game<'a>,
    {
        self.running.set(true);
        let mut previous = Instant::now();
        let update_duration = Duration::from_millis(self.target_ms_per_update);
        let mut lag = Duration::from_millis(0);
        while self.running.get() {
            let now = Instant::now();
            let elapsed = now.duration_since(previous);
            lag += elapsed;
            previous = now;

            while lag >= update_duration {
                self.update_input();
                game.update();
                lag = match lag.checked_sub(update_duration) {
                    Some(x) => x,
                    None => Duration::from_millis(0),
                };
            }

            self.render_system.render();
        }
    }

    pub fn stop(&mut self) {
        self.running.set(false);
    }

    fn update_input(&mut self) {
        self.input_system.update();
        self.input_system.handle_events(&mut |event| match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => self.running.set(false),
            _ => {}
        });
    }
}
