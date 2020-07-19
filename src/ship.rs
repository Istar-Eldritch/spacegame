use crate::{camera::CAM_HEIGHT, sprite_loader::Sprites};
use amethyst::{
    core::Transform,
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
        .build();
}

/// This system handles the movement and rotation of the ship and camera
#[derive(SystemDesc)]
pub struct ShipSystem;

impl<'s> System<'s> for ShipSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Ship>,
        ReadStorage<'s, Camera>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, ship, camera, input): Self::SystemData) {
        let mut rotation: Option<f32> = None;
        let mut movement: Option<f32> = None;
        let delta_degrees: f32 = 1.0;
        for (_ship, transform) in (&ship, &mut transforms).join() {
            if let Some(true) = input.action_is_down("turn_left") {
                rotation = Some(-delta_degrees);
                transform.append_rotation_z_axis(-delta_degrees.to_radians());
            }
            if let Some(true) = input.action_is_down("turn_right") {
                rotation = Some(delta_degrees);
                transform.append_rotation_z_axis(delta_degrees.to_radians());
            }
            if let Some(true) = input.action_is_down("accelerate") {
                movement = Some(1.0);
                transform.move_up(1.0);
            }
            if let Some(true) = input.action_is_down("break") {
                movement = Some(-1.0);
                transform.move_down(1.0);
            }
        }

        for (_camera, transform) in (&camera, &mut transforms).join() {
            // if let Some(position) = position {
            if let Some(rotation) = rotation {
                // Rotates the camera using the position of the ship as a reference.
                // TODO: There has to be a way of doing this using oly one tranformation instead of three.
                transform.move_down((CAM_HEIGHT * 0.5) - 10.0);
                transform.append_rotation_z_axis(rotation.to_radians());
                transform.move_up((CAM_HEIGHT * 0.5) - 10.0);
            }
            if let Some(movement) = movement {
                if movement > 0.0 {
                    transform.move_up(movement);
                } else {
                    transform.move_down(-movement);
                }
            }
        }
    }
}
