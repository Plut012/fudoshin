use bevy::prelude::*;
use bevy_framepace::{FramepacePlugin, FramepaceSettings, Limiter};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

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
        // Debug inspector (F1 to toggle)
        .add_plugins(WorldInspectorPlugin::new())
        // Game plugins
        .add_plugins(plugins::core_game::CoreGamePlugin)
        // Setup
        .add_systems(Startup, setup)
        .run();
}

/// Initial scene setup
fn setup(mut commands: Commands, mut framepace: ResMut<FramepaceSettings>) {
    use components::breath::*;
    use components::character::*;
    use components::combat::*;
    use components::guard::*;
    use components::health::*;
    use components::initiative::*;
    use components::state::*;
    use systems::chain::ChainState;
    use systems::momentum::Momentum;
    use systems::pressure::Pressure;

    // Lock to 60 FPS
    framepace.limiter = Limiter::from_framerate(60.0);

    // Initialize match state (starts with countdown)
    commands.insert_resource(MatchState::default());

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

    // Spawn Player 1 (red rectangle, left side)
    commands.spawn((
        Character,
        Player::One,
        CharacterState::Idle,
        MaxSpeed(300.0),
        Velocity::default(),
        Hurtbox::default(),  // Phase 2: Add hurtbox for collision
        GuardMeter::default(), // Phase 2: Add guard meter
        Initiative::default(), // Phase 3: Add initiative tracking
        Pressure::default(),   // Phase 3: Add pressure tracking
        ChainState::default(), // Phase 3: Add chain tracking
        Momentum::default(),   // Phase 3: Add momentum tracking
        Health::default(),     // Phase 4: Add health tracking
        Breath::default(),     // Phase 4: Add breath tracking
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.9, 0.2, 0.2),  // Red
                custom_size: Some(Vec2::new(60.0, 120.0)),
                ..default()
            },
            transform: Transform::from_xyz(-300.0, 0.0, 0.0),
            ..default()
        },
    ));

    // Spawn Player 2 (blue rectangle, right side)
    commands.spawn((
        Character,
        Player::Two,
        CharacterState::Idle,
        MaxSpeed(300.0),
        Velocity::default(),
        Hurtbox::default(),  // Phase 2: Add hurtbox for collision
        GuardMeter::default(), // Phase 2: Add guard meter
        Initiative::default(), // Phase 3: Add initiative tracking
        Pressure::default(),   // Phase 3: Add pressure tracking
        ChainState::default(), // Phase 3: Add chain tracking
        Momentum::default(),   // Phase 3: Add momentum tracking
        Health::default(),     // Phase 4: Add health tracking
        Breath::default(),     // Phase 4: Add breath tracking
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.2, 0.4, 0.9),  // Blue
                custom_size: Some(Vec2::new(60.0, 120.0)),
                ..default()
            },
            transform: Transform::from_xyz(300.0, 0.0, 0.0),
            ..default()
        },
    ));

    info!("Fudoshin initialized - Press F1 to toggle inspector");
    info!("P1: WASD to move | P2: Arrow keys to move");
}
