#![feature(extern_prelude)]
extern crate sdl2;

pub mod core;
pub mod systems;

use systems::render::Texture;

pub trait Game {
    fn new() -> Self;
    fn update(&mut self) -> () {}
}

macro_rules! main {
    ($tgame:ident) => {
        fn main() {
            let mut engine = core::BlightCore::new();
            // let renderer = engine.get_render();
            // let mut textures: (Texture, Texture);
            // unsafe {
            //     textures = create_game_textures(renderer);
            // }

            let mut game = $tgame::new();
            engine.run(&mut game)
        }

        unsafe fn create_game_textures<'a>(
            renderer: *mut systems::render::RenderSystem,
        ) -> (Texture<'a>, Texture<'a>) {
            let mut texture1 = (*renderer).create_texture(&(50, 50));
            let mut texture2 = (*renderer).create_texture(&(50, 50));
            (texture1, texture2)
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_of_life() {
        struct GameOfLife;
        impl Game for GameOfLife {
            fn new() -> Self {
                GameOfLife {}
            }

            fn update(&mut self) {
                println!("Hii");
            }
        }
        main!(GameOfLife);
        main();
    }

}
