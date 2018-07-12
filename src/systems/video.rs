use sdl2::{render, video, Sdl, VideoSubsystem};

type Window = video::Window;
type Canvas = render::Canvas<Window>;

pub struct VideoSystem {
    video_subsystem: Box<VideoSubsystem>,
    canvas: Canvas,
}

pub enum VideoError {
    NotInitialised,
}

impl VideoSystem {
    pub fn new(sdl_context: &Sdl) -> VideoSystem {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("Blight Engine", 800, 600)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        VideoSystem {
            video_subsystem: Box::new(video_subsystem),
            canvas: canvas,
        }
    }

    pub fn get_canvas<'a>(&'a mut self) -> &'a mut Canvas {
        &mut self.canvas
    }

    pub fn clear<'a>(&'a mut self) {
        self.canvas.clear();
        self.canvas.present();
    }
}
