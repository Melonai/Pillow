use crate::thing::Thing;

use sdl2::Sdl;
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;

pub struct Renderer {
    canvas: WindowCanvas
}

impl Renderer {
    pub fn new(sdl: Sdl, window_title: &str, window_width: u32, window_height: u32) -> Self {
        let video_subsystem = sdl.video().expect("Failed to init SDL2 video subsystem.");

        let window = video_subsystem
            .window(window_title, window_width, window_height)
            .position_centered()
            .build()
            .expect("Failed to open window.");

        let canvas = window
            .into_canvas()
            .target_texture()
            .present_vsync()
            .build()
            .expect("Failed to build window canvas.");

        Renderer { canvas }
    }

    pub fn draw_batch(&mut self, instances_to_draw: Vec<&Thing>) {
        self.canvas.set_draw_color(Color::RGB(18, 18, 35));
        self.canvas.clear();
        for instance in instances_to_draw {
            instance.draw(&mut self.canvas);
        }
        self.canvas.present();
    }
}