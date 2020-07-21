use crate::components::Speed;
use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, ReadStorage, WriteStorage},
    shred::{System, SystemData},
};

/// Recalculates the position of the entity using its speed.
#[derive(SystemDesc)]
pub struct PositionSystem;

impl<'s> System<'s> for PositionSystem {
    type SystemData = (WriteStorage<'s, Transform>, ReadStorage<'s, Speed>);

    fn run(&mut self, (mut transforms, speed): Self::SystemData) {
        for (transform, speed) in (&mut transforms, &speed).join() {
            transform.prepend_translation(speed.0);
        }
    }
}
