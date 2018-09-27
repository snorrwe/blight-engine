#[macro_use(blight_main)]
extern crate blight;

use std::collections::BTreeMap;
use std::time::{Duration, Instant};

use blight::core::BlightCore;
use blight::systems::input::{Event, Keycode, MouseButton};
use blight::systems::render::{Color, Rect, RenderComponent, RenderSystem, Texture, WINDOW_SIZE};
use blight::Game;

const PLAYGROUND_WIDTH: u32 = 49;
const PLAYGROUND_HEIGHT: u32 = 40;
const CELL_SIZE: u32 = WINDOW_SIZE.0 / PLAYGROUND_WIDTH;

struct GameOfLife<'a> {
    engine: *mut BlightCore<'a>,
    cell_texture: Texture<'a>,
    playground: [bool; (PLAYGROUND_WIDTH * PLAYGROUND_HEIGHT) as usize],
    playing: bool,
    cells: BTreeMap<usize, RenderComponent<'a>>,
    last_update: Instant,
    game_speed: Duration,
}

impl<'a> Game<'a> for GameOfLife<'a> {
    fn new(engine: *mut BlightCore<'a>) -> Self {
        unsafe {
            let renderer = (*engine).get_render();
            (*renderer).set_background_color(Some(Color::RGB(255, 255, 255)));
        }
        GameOfLife {
            cell_texture: GameOfLife::create_game_textures(engine),
            engine: engine,
            playground: [false; (PLAYGROUND_WIDTH * PLAYGROUND_HEIGHT) as usize],
            playing: false,
            cells: BTreeMap::new(),
            last_update: Instant::now(),
            game_speed: Duration::from_millis(500),
        }
    }

    fn update(&mut self) {
        unsafe {
            self.handle_input();
            let now = Instant::now();
            if now - self.last_update > self.game_speed {
                self.update_world();
                self.last_update = now;
            }
            self.render_playground();
        }
    }
}

impl<'a> GameOfLife<'a> {
    pub fn get(&mut self, x: i32, y: i32) -> Option<&bool> {
        match self.get_mut(x, y) {
            Some(x) => Some(x),
            None => None,
        }
    }

    pub fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut bool> {
        if x >= 0 && y >= 0 && (x as u32) < PLAYGROUND_WIDTH && (y as u32) < PLAYGROUND_HEIGHT {
            Some(&mut self.playground[(x as u32 + (y as u32) * PLAYGROUND_WIDTH) as usize])
        } else {
            None
        }
    }

    fn create_game_textures(engine: *mut BlightCore<'a>) -> Texture<'a> {
        unsafe {
            let renderer = (*engine).get_render();
            let mut texture = (*renderer).create_texture(&(CELL_SIZE, CELL_SIZE));
            GameOfLife::init_game_textures(&mut texture, renderer);
            texture
        }
    }

    unsafe fn init_game_textures(texture: &mut Texture<'a>, renderer: *mut RenderSystem) {
        (*renderer)
            .get_canvas_mut()
            .with_texture_canvas(texture, |texture| {
                texture.set_draw_color(Color::RGB(0, 0, 0));
                texture.clear();
            })
            .unwrap();
    }

    fn update_world(&mut self) {
        if !self.playing {
            return;
        }
        let mut new_playground = self.playground;
        for (i, cell) in new_playground.iter_mut().enumerate() {
            let i = i as u32;
            let (x, y) = (i % PLAYGROUND_WIDTH, i / PLAYGROUND_WIDTH);
            let mut count: u32 = 0;
            for i in -1..2 {
                for j in -1..2 {
                    if !(i == 0 && j == 0) {
                        let peek_x: i32 = (x as i32) + i;
                        let peek_y: i32 = (y as i32) + j;
                        if let Some(true) = self.get(peek_x, peek_y) {
                            count += 1;
                        }
                    }
                }
            }
            if count > 3 || count < 2 {
                *cell = false;
            } else if count == 3 {
                *cell = true;
            }
        }
        self.playground = new_playground;
    }

    unsafe fn handle_input(&mut self) {
        (*self.engine)
            .get_input()
            .handle_events(&mut |event| match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    repeat: false,
                    ..
                } => {
                    self.playing = !self.playing;
                }
                Event::MouseButtonDown {
                    x,
                    y,
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    let x = ((*x as u32) / CELL_SIZE) as i32;
                    let y = ((*y as u32) / CELL_SIZE) as i32;
                    if let Some(cell) = self.get_mut(x, y) {
                        *cell = !*cell;
                    }
                }
                _ => {}
            });
    }

    unsafe fn render_playground(&mut self) {
        let renderer = (*self.engine).get_render();
        for (i, cell) in self.playground.iter().enumerate() {
            if !*cell {
                self.cells.remove(&i);
                continue;
            }
            if !self.cells.contains_key(&i) {
                let mut component = (*renderer).create_component();
                let x = i as u32;
                component.position = Rect::new(
                    ((x % PLAYGROUND_WIDTH) * CELL_SIZE) as i32,
                    ((x / PLAYGROUND_WIDTH) * CELL_SIZE) as i32,
                    CELL_SIZE,
                    CELL_SIZE,
                );
                self.cells.insert(i, component);
            }
            let mut component = self.cells.get_mut(&i).unwrap();
            component.texture = &self.cell_texture;
        }
    }
}

blight_main!(GameOfLife);
