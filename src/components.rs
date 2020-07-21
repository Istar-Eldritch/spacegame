use amethyst::{
    core::math::Vector3,
    ecs::{Component, DenseVecStorage, FlaggedStorage},
};

/// How fast and the direction of movement of an object
pub struct Speed(pub Vector3<f32>);

impl Default for Speed {
    fn default() -> Self {
        Speed(Vector3::new(0.0, 0.0, 0.0))
    }
}

impl Component for Speed {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}
