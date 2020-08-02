use crate::thing::Thing;
use crate::context::WorldContext;

pub struct World {
    objects: Vec<Thing>,
    width: u32,
    height: u32
}

impl World {
    pub fn new(width: u32, height: u32) -> Self {
        World { objects: Vec::new(), width, height}
    }

    pub fn step(&mut self) {
        let world_context = WorldContext {width: self.width, height: self.height};
        for thing in self.objects.iter_mut() {
            thing.step(world_context);
        }
    }

    pub fn add_thing(&mut self, thing: Thing) {
        self.objects.push(thing);
    }

    pub fn get_things(&self) -> Vec<&Thing> {
        self.objects.iter().collect()
    }
}