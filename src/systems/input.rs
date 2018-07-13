use sdl2::{self, Sdl};

pub type EventPump = sdl2::EventPump;
pub type Event = sdl2::event::Event;

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

    pub fn handle_events(&self, callback: &(Fn(&Event) -> ())) {
        for event in self.events.iter() {
            callback(&event);
        }
    }
}