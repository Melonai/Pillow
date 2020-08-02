pub mod context;

mod sprite;
mod thing;
mod world;
mod renderer;

use crate::renderer::Renderer;

use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use world::World;
use thing::Thing;

pub use sdl2::rect::Rect;
pub use sdl2::pixels::Color;

pub use thing::Existence;
pub use sprite::Sprite;

pub struct Pillow {
    event_pump: EventPump,
    world: World,
    renderer: Renderer,
    time: u64
}

impl Pillow {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let sdl_context = sdl2::init().expect("Failed to init SDL2.");

        let event_pump = sdl_context.event_pump().expect("Failed creating SDL2 event loop.");

        let world = World::new(width, height);
        let renderer = Renderer::new(sdl_context, title, width, height);

        Pillow { event_pump, world, renderer, time: 0 }
    }

    pub fn instantiate<T: Existence + 'static>(&mut self, object: T, x: f32, y: f32) {
        self.world.add_thing(Thing::new(Box::new(object), x, y));
    }

    pub fn run(&mut self) {
        'running: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => {}
                }
            }

            self.world.step();
            self.renderer.draw_batch(self.world.get_things());

            self.time += 1;
        }
    }
}