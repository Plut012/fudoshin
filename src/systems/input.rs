use bevy::prelude::*;
use crate::components::character::Player;

/// Raw input state for each player
#[derive(Debug, Clone, Copy, Default)]
pub struct PlayerInput {
    pub movement: Vec2,      // -1 to 1 for left/right (Y unused for now)
    pub light_attack: bool,
    pub heavy_attack: bool,
    pub grab: bool,
    pub block: bool,
    pub step: bool,          // Quick dash
    pub backdash: bool,
}

/// Get Player 1 input from keyboard
pub fn get_p1_input(keys: &ButtonInput<KeyCode>) -> PlayerInput {
    let mut input = PlayerInput::default();

    // Movement (WASD)
    if keys.pressed(KeyCode::KeyA) {
        input.movement.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        input.movement.x += 1.0;
    }

    // Actions (JKL)
    input.light_attack = keys.just_pressed(KeyCode::KeyJ);
    input.heavy_attack = keys.just_pressed(KeyCode::KeyK);
    input.grab = keys.just_pressed(KeyCode::KeyL);
    input.block = keys.pressed(KeyCode::KeyI);  // I for block

    // Movement options (Shift + direction)
    let shift = keys.pressed(KeyCode::ShiftLeft);
    if shift && input.movement.x != 0.0 {
        input.step = true;
    }
    if shift && keys.just_pressed(KeyCode::KeyS) {
        input.backdash = true;
    }

    input
}

/// Get Player 2 input from keyboard
pub fn get_p2_input(keys: &ButtonInput<KeyCode>) -> PlayerInput {
    let mut input = PlayerInput::default();

    // Movement (Arrow keys)
    if keys.pressed(KeyCode::ArrowLeft) {
        input.movement.x -= 1.0;
    }
    if keys.pressed(KeyCode::ArrowRight) {
        input.movement.x += 1.0;
    }

    // Actions (Numpad 1/2/3)
    input.light_attack = keys.just_pressed(KeyCode::Numpad1);
    input.heavy_attack = keys.just_pressed(KeyCode::Numpad2);
    input.grab = keys.just_pressed(KeyCode::Numpad3);
    input.block = keys.pressed(KeyCode::Numpad0);

    // Movement options (RShift + direction)
    let shift = keys.pressed(KeyCode::ShiftRight);
    if shift && input.movement.x != 0.0 {
        input.step = true;
    }
    if shift && keys.just_pressed(KeyCode::ArrowDown) {
        input.backdash = true;
    }

    input
}

/// Resource to store current frame's inputs
#[derive(Resource, Default)]
pub struct CurrentInputs {
    pub player_one: PlayerInput,
    pub player_two: PlayerInput,
}

/// System to update input resource each frame
pub fn update_inputs(
    keys: Res<ButtonInput<KeyCode>>,
    mut inputs: ResMut<CurrentInputs>,
) {
    inputs.player_one = get_p1_input(&keys);
    inputs.player_two = get_p2_input(&keys);
}
