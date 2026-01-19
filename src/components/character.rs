use bevy::prelude::*;

/// Marker component for characters
#[derive(Component)]
pub struct Character;

/// Player identifier (1 or 2)
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    One,
    Two,
}

/// Maximum movement speed
#[derive(Component)]
pub struct MaxSpeed(pub f32);

/// Current velocity
#[derive(Component, Default)]
pub struct Velocity(pub Vec2);
