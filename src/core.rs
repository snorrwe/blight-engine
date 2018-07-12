use super::systems::render::RenderSystem;
use super::systems::video::VideoSystem;
use sdl2;

pub struct BlightCore {
    sdl: sdl2::Sdl,
    video_system: VideoSystem,
    render_system: RenderSystem,
}

impl BlightCore {
    pub fn new() -> BlightCore {
        let sdl = sdl2::init().unwrap();
        BlightCore {
            video_system: VideoSystem::new(&sdl),
            render_system: RenderSystem::new(&sdl),
            sdl: sdl,
        }
    }

    pub fn run(&mut self) {
        use sdl2::event::Event;
        use sdl2::keyboard::Keycode;
        use sdl2::pixels::Color;
        use std::time::Duration;

        self.video_system
            .get_canvas()
            .set_draw_color(Color::RGB(255, 255, 0));
        self.video_system.clear();
        let mut event_pump = self.sdl.event_pump().unwrap();

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
            // The rest of the game loop goes here...
        }
    }
}
