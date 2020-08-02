use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub struct Sprite {
    rect: Rect,
}

impl Sprite {
    pub fn new(width: u32, height: u32) -> Self {
        Sprite {rect: Rect::new(0, 0, width, height) }
    }

    pub fn draw_onto_canvas(&self,  canvas: &mut WindowCanvas, _x: f32, _y: f32) {
        canvas.draw_rect(self.rect).unwrap();
        //TODO: Textures!
    }
}