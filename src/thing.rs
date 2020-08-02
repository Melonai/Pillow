use sdl2::render::WindowCanvas;

use crate::Sprite;
use crate::context::{StepContext, WorldContext, DrawContext};

pub struct Thing {
    pub x: f32,
    pub y: f32,
    sprite: Option<Sprite>,
    object: Box<dyn Existence>
}

impl Thing {
    pub fn new(object: Box<dyn Existence>, x: f32, y: f32) -> Self {
        Thing { x, y, sprite: object.sprite(), object }
    }

    pub fn step(&mut self, world_context: WorldContext) {
        let ctx = StepContext::new(&mut self.x, &mut self.y, world_context);
        self.object.step(ctx);
    }

    pub fn draw(&self, canvas: &mut WindowCanvas) {
        let ctx = DrawContext::new(self, self.sprite.as_ref(), canvas);
        self.object.draw(ctx);
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }
}

pub trait Existence {
    fn step(&mut self, _ctx: StepContext) {}

    fn draw(&self, mut ctx: DrawContext) {
        if let Some(existing_sprite) = ctx.sprite {
            ctx.draw_sprite(existing_sprite);
        }
    }

    fn sprite(&self) -> Option<Sprite> {
        None
    }
}