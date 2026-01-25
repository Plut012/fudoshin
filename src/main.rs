use bevy::prelude::*;
use bevy_framepace::{FramepacePlugin, FramepaceSettings, Limiter};

mod components;
mod data;
mod events;
mod plugins;
mod resources;
mod systems;

fn main() {
    App::new()
        // Window and rendering setup
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Fudoshin - The Immovable Mind".to_string(),
                resolution: (1280.0, 720.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        // Frame pacing - locked 60 FPS
        .add_plugins(FramepacePlugin)
        // Game plugins
        .add_plugins(plugins::core_game::CoreGamePlugin)
        // Setup
        .add_systems(Startup, setup)
        .run();
}

/// Initial scene setup - only persistent elements (camera, stage)
fn setup(mut commands: Commands, mut framepace: ResMut<FramepaceSettings>) {
    // Lock to 60 FPS
    framepace.limiter = Limiter::from_framerate(60.0);

    // Camera
    commands.spawn(Camera2dBundle::default());

    // Stage boundaries (visual reference for now)
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgb(0.2, 0.2, 0.25),
            custom_size: Some(Vec2::new(1000.0, 600.0)),
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, -1.0),
        ..default()
    });

    info!("Fudoshin initialized");
    info!("Game starts in Character Select - press J or Numpad1 to ready up");
}
