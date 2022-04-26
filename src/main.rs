#![recursion_limit = "512"]

use amethyst::{
    core::{math::Vector3, transform::TransformBundle},
    ecs::{Component, DenseVecStorage, FlaggedStorage},
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderDebugLines, RenderingBundle,
    },
    utils::application_root_dir,
};

mod camera;
mod components;
mod debug_lines;
mod ship;
mod sprite_loader;
mod systems;

use camera::initialise_camera;
use debug_lines::initialise_debug;
use ship::{initialise_ship, Ship, ShipMovementSystem};
use sprite_loader::load_sprites;
use systems::PositionSystem;

pub struct GameState {}

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let sprites = load_sprites(data.world);

        data.world.insert(sprites);

        data.world.register::<Ship>();

        initialise_debug(data.world);
        initialise_ship(data.world);
        initialise_camera(data.world);
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let display_config = app_root.join("resources").join("display_config.ron");

    let render_bundle = RenderingBundle::<DefaultBackend>::new()
        .with_plugin(
            RenderToWindow::from_config_path(display_config)?.with_clear([0.0, 0.0, 0.0, 1.0]),
        )
        .with_plugin(RenderFlat2D::default())
        .with_plugin(RenderDebugLines::default());

    let bindings_config = app_root.join("resources").join("bindings.ron");

    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(bindings_config)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(render_bundle)?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(PositionSystem, "position_system", &[])
        .with(
            ShipMovementSystem,
            "ship_movement_system",
            &["input_system", "position_system"],
        );

    let assets_dir = app_root.join("resources");

    let mut game = Application::new(assets_dir, GameState {}, game_data)?;

    game.run();

    Ok(())
}
