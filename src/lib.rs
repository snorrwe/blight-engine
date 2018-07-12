#![feature(extern_prelude)]
extern crate sdl2;

pub mod core;
pub mod systems;

macro_rules! main {
    () => {
        let mut engine = BlightCore::new();
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let mut engine = core::BlightCore::new();
        engine.run()
    }

}
