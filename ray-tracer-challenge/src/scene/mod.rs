use crate::draw::canvas::Canvas;

use self::{
    camera::{Camera, RenderOpts},
    world::World,
};

pub mod camera;
pub mod intersect;
pub mod light;
pub mod material;
pub mod object;
pub mod pattern;
pub mod ray;
pub mod transformation;
pub mod world;

pub struct Scene {
    pub camera: Camera,
    pub world: World,
}

impl Scene {
    pub fn render(&self, opts: &RenderOpts) -> Canvas {
        self.camera.render(&self.world, opts)
    }
}
