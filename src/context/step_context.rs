use crate::context::WorldContext;

pub struct StepContext<'a> {
    pub x: &'a mut f32,
    pub y: &'a mut f32,
    pub world_width: u32,
    pub world_height: u32
}

impl<'a> StepContext<'a> {
    pub fn new(x: &'a mut f32, y: &'a mut f32, world: WorldContext) -> Self {
        StepContext {
            x,
            y,
            world_width: world.width,
            world_height: world.height
        }
    }

    pub fn set_pos(&mut self, x: f32, y: f32) {
        *self.x = x;
        *self.y = y;
    }
}