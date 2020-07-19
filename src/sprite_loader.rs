use amethyst::{
    assets::{AssetStorage, Loader},
    prelude::WorldExt,
    renderer::{ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    shred::World,
};

pub struct Sprites {
    pub ship: SpriteRender,
}

/// Loads and splits the `logo.png` image asset into 3 sprites,
/// which will then be assigned to entities for rendering them.
///
/// The provided `world` is used to retrieve the resource loader.
pub fn load_sprites(world: &mut World) -> Sprites {
    // Load the texture for our sprites. We'll later need to
    // add a handle to this texture to our `SpriteRender`s, so
    // we need to keep a reference to it.
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load("sprites.png", ImageFormat::default(), (), &texture_storage)
    };

    // Load the spritesheet definition file, which contains metadata on our
    // spritesheet texture.
    let sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            "sprites.ron",
            SpriteSheetFormat(texture_handle),
            (),
            &sheet_storage,
        )
    };

    Sprites {
        ship: SpriteRender {
            sprite_sheet: sheet_handle,
            sprite_number: 0,
        },
    }
}
