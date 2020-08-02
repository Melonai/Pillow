use sdl2::render::WindowCanvas;
use crate::sprite::Sprite;
use crate::thing::Thing;
use crate::{Rect, Color};

pub struct DrawContext<'a> {
    pub x: f32,
    pub y: f32,
    pub sprite: Option<&'a Sprite>,
    canvas: &'a mut WindowCanvas
}

impl<'a> DrawContext<'a> {
    pub fn new(thing: &Thing, sprite: Option<&'a Sprite>, canvas: &'a mut WindowCanvas) -> Self {
        DrawContext {
            x: thing.x,
            y: thing.y,
            sprite,
            canvas
        }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn draw_rect(&mut self, rect: Rect, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.fill_rect(rect).unwrap();
    }

    pub fn draw_sprite(&mut self, sprite: &Sprite) {
        sprite.draw_onto_canvas(self.canvas, self.x, self.y);
    }
}