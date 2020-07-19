use amethyst::{
    prelude::{Builder, WorldExt},
    renderer::{debug_drawing::DebugLinesComponent, palette::Srgba},
    shred::World,
    window::ScreenDimensions,
};

pub fn initialise_debug(world: &mut World) {
    // Setup debug lines as a component and add lines to render axis&grid
    let mut debug_lines_component = DebugLinesComponent::new();

    let (screen_w, screen_h) = {
        let screen_dimensions = world.read_resource::<ScreenDimensions>();
        (screen_dimensions.width(), screen_dimensions.height())
    };

    for y in (0..(screen_h as u16)).step_by(8).map(f32::from) {
        let y_pos = y - 10.0;
        debug_lines_component.add_line(
            [-screen_w * 0.5, y_pos, 0.0].into(),
            [screen_w * 0.5, y_pos, 0.0].into(),
            Srgba::new(0.3, 0.3, 0.3, 1.0),
        );
    }

    for x in (0..(screen_w as u16)).step_by(8).map(f32::from) {
        debug_lines_component.add_line(
            [x - screen_w * 0.5, -10.0, 0.0].into(),
            [x - screen_w * 0.5, screen_h, 0.0].into(),
            Srgba::new(0.3, 0.3, 0.3, 1.0),
        );
    }

    world.create_entity().with(debug_lines_component).build();
}
