#![feature(extern_prelude)]
extern crate sdl2;
#[macro_use(blight_main)]
extern crate blight;

use std::collections::BTreeMap;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};

use blight::core::BlightCore;
use blight::systems::render::{Canvas, RenderComponent, RenderSystem, Texture, WINDOW_SIZE};
use blight::Game;

const PLAYGROUND_WIDTH: u32 = 49;
const PLAYGROUND_HEIGHT: u32 = 40;
const CELL_SIZE: u32 = WINDOW_SIZE.0 / PLAYGROUND_WIDTH;
const BLINK_THRESHOLD: u8 = 60;

struct GameOfLife<'a> {
    engine: *mut BlightCore<'a>,
    textures: (Texture<'a>, Texture<'a>),
    playground: [bool; (PLAYGROUND_WIDTH * PLAYGROUND_HEIGHT) as usize],
    playing: bool,
    blinks: u8,
    cells: BTreeMap<usize, RenderComponent<'a>>,
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

    fn create_game_textures(engine: *mut BlightCore<'a>) -> (Texture<'a>, Texture<'a>) {
        unsafe {
            let renderer = (*engine).get_render();
            let mut texture1 = (*renderer).create_texture(&(CELL_SIZE, CELL_SIZE));
            let mut texture2 = (*renderer).create_texture(&(CELL_SIZE, CELL_SIZE));
            GameOfLife::init_game_textures(&mut texture1, &mut texture2, renderer);
            (texture1, texture2)
        }
    }

    unsafe fn init_game_textures(
        texture1: &mut Texture<'a>,
        texture2: &mut Texture<'a>,
        renderer: *mut RenderSystem,
    ) {
        enum CellColor {
            White,
            Yellow,
        }
        let textures = vec![(texture1, CellColor::Yellow), (texture2, CellColor::White)];
        (*renderer)
            .get_canvas_mut()
            .with_multiple_texture_canvas(textures.iter(), |texture, context| {
                texture.set_draw_color(Color::RGB(0, 0, 0));
                texture.clear();
                for i in 0..CELL_SIZE {
                    for j in 0..CELL_SIZE {
                        let (i, j) = (i as i32, j as i32);
                        if let CellColor::Yellow = *context {
                            GameOfLife::color_texture(
                                texture,
                                i,
                                j,
                                (Color::RGB(255, 255, 0), Color::RGB(200, 200, 0)),
                                (4, 9),
                            );
                        }
                        GameOfLife::color_texture(
                            texture,
                            i,
                            j,
                            (Color::RGB(192, 192, 192), Color::RGB(64, 64, 64)),
                            (7, 5),
                        );
                    }
                }
            })
            .unwrap();
    }

    fn color_texture(
        texture: &mut Canvas,
        i: i32,
        j: i32,
        colors: (Color, Color),
        modulos: (i32, i32),
    ) {
        if (i + j) % modulos.0 == 0 {
            texture.set_draw_color(colors.0);
            texture.draw_point(Point::new(i, j)).unwrap();
        } else if (i + j * 2) % modulos.0 == 0 {
            texture.set_draw_color(colors.1);
            texture.draw_point(Point::new(i, j)).unwrap();
        }
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
                component.set_position(Rect::new(
                    ((x % PLAYGROUND_WIDTH) * CELL_SIZE) as i32,
                    ((x / PLAYGROUND_WIDTH) * CELL_SIZE) as i32,
                    CELL_SIZE,
                    CELL_SIZE,
                ));
                self.cells.insert(i, component);
            }
            let mut component = self.cells.get_mut(&i).unwrap();
            let texture = if self.blinks > BLINK_THRESHOLD / 2 {
                &self.textures.0
            } else {
                &self.textures.1
            };
            component.set_texture(texture);
        }
        self.blinks = (self.blinks + 1) % BLINK_THRESHOLD;
    }
}

impl<'a> Game<'a> for GameOfLife<'a> {
    fn new(engine: *mut BlightCore<'a>) -> Self {
        GameOfLife {
            textures: GameOfLife::create_game_textures(engine),
            engine: engine,
            playground: [false; (PLAYGROUND_WIDTH * PLAYGROUND_HEIGHT) as usize],
            playing: false,
            blinks: 0,
            cells: BTreeMap::new(),
        }
    }

    fn update(&mut self) {
        unsafe {
            self.handle_input();
            self.update_world();
            self.render_playground();
        }
    }
}

blight_main!(GameOfLife);
