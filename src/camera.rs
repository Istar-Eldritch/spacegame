use crate::components::Speed;
use amethyst::{
    core::Transform,
    prelude::{Builder, WorldExt},
    renderer::Camera,
    shred::World,
};

pub const CAM_HEIGHT: f32 = 144.0;
pub const CAM_WIDTH: f32 = 160.0;

pub fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, CAM_HEIGHT * 0.5 - 10.0, 5.0);

    world
        .create_entity()
        .with(Camera::standard_2d(CAM_WIDTH, CAM_HEIGHT))
        .with(transform)
        .with(Speed::default())
        .build();
}
