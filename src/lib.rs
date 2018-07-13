#![feature(extern_prelude)]
extern crate sdl2;

pub mod components;
pub mod core;
pub mod systems;

macro_rules! main {
    () => {
        fn main() {
            use components::render::RenderComponent;
            use systems::render::Texture;

            let mut engine = core::BlightCore::new();
            let mut renderer = engine.get_render();
            let mut texture: Texture;
            let mut cell: RenderComponent;
            unsafe {
                texture = (*renderer).create_texture(&(50, 50));
                cell = RenderComponent::new(&mut texture);
            }
            engine.run()
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_of_life() {
        main!();
        main();
    }

}
