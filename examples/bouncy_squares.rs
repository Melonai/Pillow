use rand::Rng;

use pillow::{Pillow, Rect, Color, Existence};
use pillow::context::{StepContext, DrawContext};

struct Square {
    speed: f32,
    horizontal_dir: i8,
    vertical_dir: i8,
}

impl Existence for Square {
    fn step(&mut self, mut ctx: StepContext) {
        let current_x = *ctx.x;
        let current_y = *ctx.y;
        let world_width = ctx.world_width as f32 - 50.0;
        let world_height = ctx.world_height as f32 - 50.0;

        if current_x > world_width || current_x < 0.0 {
            self.horizontal_dir *= -1;
        }
        if current_y > world_height || current_y < 0.0 {
            self.vertical_dir *= -1;
        }

        ctx.set_pos(
            current_x + self.speed * self.horizontal_dir as f32,
            current_y + self.speed * self.vertical_dir as f32
        );
    }

    fn draw(&self, mut ctx: DrawContext) {
        let black_rect = Rect::new(ctx.x as i32, ctx.y as i32, 50, 50);

        let border_width = 5;
        let mut border_rect = black_rect.clone();
        border_rect.resize(black_rect.width() + border_width * 2, black_rect.height() + border_width * 2);
        border_rect.offset(-(border_width as i32), -(border_width as i32));

        ctx.draw_rect(border_rect, Color::RGB(0, 127, 255));
        ctx.draw_rect(black_rect, Color::RGB(18, 18, 35));
    }
}

fn main() {
    let mut pillow = Pillow::new("Bouncy Squares", 1280, 720);
    let mut rand_gen = rand::thread_rng();

    for _ in 0..10 {
        let speed = rand_gen.gen_range(1.0, 5.0);
        let horizontal_dir = rand_gen.gen::<bool>() as i8 * 2 - 1;
        let vertical_dir = rand_gen.gen::<bool>() as i8 * 2 - 1;

        let x = rand_gen.gen_range(0.0, 1280.0 - 50.0);
        let y = rand_gen.gen_range(0.0, 720.0 - 50.0);

        let square = Square {speed, horizontal_dir, vertical_dir};
        pillow.instantiate(square, x, y);
    }

    pillow.run();
}