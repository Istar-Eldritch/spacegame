use crate::{camera::CAM_HEIGHT, components::Speed, sprite_loader::Sprites};
use amethyst::{
    core::{
        math::{Rotation3, Vector3},
        Transform,
    },
    derive::SystemDesc,
    ecs::{Component, DenseVecStorage, Join, ReadStorage, WriteStorage},
    input::{InputHandler, StringBindings},
    prelude::{Builder, WorldExt},
    renderer::Camera,
    shred::{Read, System, SystemData, World},
};

pub struct Ship {}

impl Ship {
    fn new() -> Self {
        Ship {}
    }
}

impl Component for Ship {
    type Storage = DenseVecStorage<Self>;
}

pub fn initialise_ship(world: &mut World) {
    let mut ship_transform = Transform::default();
    let ship_sprite = world
        .try_fetch::<Sprites>()
        .expect("Couldn't load sprite sheet")
        .ship
        .clone();

    ship_transform.set_translation_xyz(0.0, 0.0, 0.0);

    world
        .create_entity()
        .with(Ship::new())
        .with(ship_transform)
        .with(ship_sprite)
        .with(Speed::default())
        .build();
}

/// This system handles the movement and rotation of the ship and camera
#[derive(SystemDesc)]
pub struct ShipSystem;

impl<'s> System<'s> for ShipSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Speed>,
        ReadStorage<'s, Ship>,
        ReadStorage<'s, Camera>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, mut speeds, ship, camera, input): Self::SystemData) {
        let delta_degrees: f32 = 1.0;
        let delta_speed: Vector3<f32> = Vector3::new(0.0, 0.01, 0.0);

        for (_ship, transform, speed) in (&ship, &mut transforms, &mut speeds).join() {
            if let Some(true) = input.action_is_down("turn_left") {
                transform.append_rotation_z_axis(-delta_degrees.to_radians());
            }
            if let Some(true) = input.action_is_down("turn_right") {
                transform.append_rotation_z_axis(delta_degrees.to_radians());
            }

            let rot = transform.rotation();
            let delta_speed = rot.transform_vector(&delta_speed);
            if let Some(true) = input.action_is_down("accelerate") {
                speed.0 += delta_speed;
            }
            if let Some(true) = input.action_is_down("break") {
                let rot180 = Rotation3::new(speed.0.normalize().scale(180.0_f32.to_radians()));
                let neg_speed = rot180.transform_vector(&speed.0);
                speed.0 = neg_speed.scale(0.95);
            }
        }

        for (_camera, transform, speed) in (&camera, &mut transforms, &mut speeds).join() {
            if let Some(true) = input.action_is_down("turn_left") {
                // Rotates the camera using the position of the ship as a reference.
                // TODO: There has to be a way of doing this using oly one tranformation instead of three.
                transform.move_down((CAM_HEIGHT * 0.5) - 10.0);
                transform.append_rotation_z_axis(-delta_degrees.to_radians());
                transform.move_up((CAM_HEIGHT * 0.5) - 10.0);
            }
            if let Some(true) = input.action_is_down("turn_right") {
                // Rotates the camera using the position of the ship as a reference.
                // TODO: There has to be a way of doing this using oly one tranformation instead of three.
                transform.move_down((CAM_HEIGHT * 0.5) - 10.0);
                transform.append_rotation_z_axis(delta_degrees.to_radians());
                transform.move_up((CAM_HEIGHT * 0.5) - 10.0);
            }
            let rot = transform.rotation();

            if let Some(true) = input.action_is_down("accelerate") {
                let nv = rot.transform_vector(&delta_speed);
                speed.0 += nv;
            }
            if let Some(true) = input.action_is_down("break") {
                let rot180 = Rotation3::new(speed.0.normalize().scale(180.0_f32.to_radians()));
                let neg_speed = rot180.transform_vector(&speed.0);
                speed.0 = neg_speed.scale(0.95);
            }
        }
    }
}
