#![feature(extern_prelude)]
extern crate sdl2;

pub mod common;
pub mod components;
pub mod core;
pub mod systems;

use core::BlightCore;

pub trait Game<'a> {
    fn new(engine: *mut BlightCore<'a>) -> Self;
    fn update(&mut self) -> () {}
}

#[macro_export]
macro_rules! blight_main {
    ($tgame:ident) => {
        fn main() {
            let mut engine = BlightCore::new();
            let mut game = $tgame::new(&mut engine as *mut BlightCore);
            engine.run(&mut game)
        }
    };
}

#[cfg(test)]
mod tests {}
