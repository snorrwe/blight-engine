use sdl2::{self, Sdl};
use std::slice::Iter;

pub type EventPump = sdl2::EventPump;

pub use sdl2::event::Event;
pub use sdl2::keyboard::Keycode;
pub use sdl2::mouse::MouseButton;

pub struct InputSystem {
    event_pump: EventPump,
    events: Vec<Event>,
}

impl InputSystem {
    pub fn new(sdl: &Sdl) -> InputSystem {
        let event_pump = sdl.event_pump().unwrap();
        let result = InputSystem {
            event_pump: event_pump,
            events: vec![],
        };
        result
    }

    pub fn update(&mut self) {
        self.events.clear();
        for event in self.event_pump.poll_iter() {
            self.events.push(event);
        }
    }

    /// Pass in a callback to handle each event in the current frame
    pub fn handle_events(&self, callback: &mut (FnMut(&Event) -> ())) {
        self.events.iter().for_each(callback)
    }

    /// Get an iterator into the events of the current frame
    pub fn iter_events(&self) -> Iter<Event> {
        self.events.iter()
    }
}
